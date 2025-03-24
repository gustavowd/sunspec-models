use std::{
    fs::File,
    io::{self, BufReader},
    net::SocketAddr,
    path::Path,
    sync::Arc
};
 
use futures::future;
use futures_time::prelude::*;
use pkcs8::der::Decode;
use rustls_pemfile::{certs, pkcs8_private_keys, ec_private_keys};
use sunspec_models::models::{Models, Model, SunspecModels, SUNSPEC_INIT_ADDR};
use sunspec_models::types::{SunspecTypes, DataTypes};
use tokio::net::TcpListener;
use tokio_modbus::{
    prelude::*,
    server::tcp::Server
};
use tokio_rustls::rustls;
use pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::TlsAcceptor;
use tokio::sync::Mutex;
use ini::Ini;

mod model_init;
use model_init::*;

fn load_certs(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    certs(&mut BufReader::new(File::open(path)?)).collect()
}

fn load_keys(path: &Path, password: Option<&str>) -> io::Result<PrivateKeyDer<'static>> {
    let expected_tag = match &password {
        Some(_) => "ENCRYPTED PRIVATE KEY",
        None => "PRIVATE KEY",
    };

    if expected_tag.eq("PRIVATE KEY") {
        let content = std::fs::read(path)?;
        if content.starts_with("-----BEGIN EC PRIVATE KEY".as_bytes()) {
            ec_private_keys(&mut BufReader::new(File::open(path)?))
            .next()
            .unwrap()
            .map(Into::into)
        }else{
            pkcs8_private_keys(&mut BufReader::new(File::open(path)?))
            .next()
            .unwrap()
            .map(Into::into)
        }
    } else {
        let content = std::fs::read(path)?;
        let mut iter = pem::parse_many(content)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?
            .into_iter()
            .filter(|x| x.tag() == expected_tag)
            .map(|x| x.contents().to_vec());

        match iter.next() {
            Some(key) => match password {
                Some(password) => {
                    let encrypted =
                        pkcs8::EncryptedPrivateKeyInfo::from_der(&key).map_err(|err| {
                            io::Error::new(io::ErrorKind::InvalidData, err.to_string())
                        })?;
                    let decrypted = encrypted.decrypt(password).map_err(|err| {
                        io::Error::new(io::ErrorKind::InvalidData, err.to_string())
                    })?;
                    let key = decrypted.as_bytes().to_vec();
                    match rustls_pemfile::read_one_from_slice(&key).expect("cannot parse private key .pem file") {
                        Some((rustls_pemfile::Item::Pkcs1Key(key), _keys)) => io::Result::Ok(key.into()),
                        Some((rustls_pemfile::Item::Pkcs8Key(key), _keys)) => io::Result::Ok(key.into()),
                        Some((rustls_pemfile::Item::Sec1Key(key), _keys)) => io::Result::Ok(key.into()),
                        _ => io::Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid key")),
                    }
                }
                None => io::Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid key")),
            },
            None => io::Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid key")),
        }
    }
}


struct Service {
    data: Arc<Mutex<Models>>,
}

impl Service {
    fn new(init_data: Arc<Mutex<Models>>) -> Self {
        Self {
            data: init_data,
        }
    }
}

impl tokio_modbus::server::Service for Service {
    type Request = SlaveRequest<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future
    {
        let res = match req.request.clone() {
            Request::ReadHoldingRegisters(addr, cnt) => {
                let mut start_addr = 0;
                let mut registers: Vec<u16> = Vec::new();
                let mut end_addr: u16 = 0;

                futures::executor::block_on(async {
                    let models = self.data.lock()
                        .timeout(futures_time::time::Duration::from_millis(50))
                        .await;
                    if let Ok(models) = models {
                        if (SUNSPEC_INIT_ADDR..SUNSPEC_INIT_ADDR + 2).contains(&addr) {
                            registers = models.id.into();
                            start_addr = SUNSPEC_INIT_ADDR;
                        }else {
                            for model in models.models.iter() {
                                if (addr >= model.start_addr) && (addr < model.end_addr) {
                                    let model_tmp = model.clone();
                                    registers = Vec::<u16>::from(model_tmp);
                                    start_addr = model.start_addr;
                                }
                            }
                        }
                        end_addr = models.models[models.models.len()-1].end_addr;
                        drop(models);
                    }
                });

                if addr >= SUNSPEC_INIT_ADDR && (addr + cnt) <= end_addr {
                    let registers = &registers[(addr-start_addr) as usize..(cnt+(addr-start_addr)) as usize];
                    Ok(Response::ReadHoldingRegisters(registers.to_vec()))
                }else {
                    Err(ExceptionCode::IllegalDataAddress)
                }
            },
            Request::WriteMultipleRegisters(addr, regs) => {
                let mut end_addr: u16 = 0;
                futures::executor::block_on(async {
                    let models = self.data.lock()
                        .timeout(futures_time::time::Duration::from_millis(50))
                        .await;
                    if let Ok(models) = models {
                        end_addr = models.models[models.models.len()-1].end_addr;
                        drop(models);
                    }
                });
                if addr >= SUNSPEC_INIT_ADDR && (addr + regs.len() as u16) <= end_addr {
                    println!("Write Multiple Registers - Addr: {} - cnt: {} - regs: {:?}", addr, regs.len(), regs);
                    futures::executor::block_on(async {
                        let models = self.data.lock()
                            .timeout(futures_time::time::Duration::from_millis(50))
                            .await;
                        if let Ok(mut models) = models {
                            for model in models.models.iter_mut() {
                                if (addr >= model.start_addr) && (addr <= (model.end_addr)) {
                                    let regs_vec = regs.to_vec();
                                    let mut model_tmp = Model::from((regs_vec, addr, regs.len() as u16, &model.clone()));
                                    model_tmp.update = true;
                                    *model = model_tmp;
                                    //println!("{:?}", model);
                                }
                            }
                            drop(models);
                        }
                    });

                    Ok(Response::WriteMultipleRegisters(addr,regs.len() as u16))
                }else {
                    Err(ExceptionCode::IllegalDataAddress)
                }
            },
            _ => Err(ExceptionCode::IllegalFunction)
        };
        future::ready(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr_result = "0.0.0.0:8802".parse();
    let socket_addr = match socket_addr_result {
        Ok(addr) => addr,
        Err(error) => panic!("Fail to parse socket address: {:?}", error),
    };

    let data: Arc<Mutex<Models>> = Arc::new(Mutex::new(Models::new()));
    let mut new_model = data.lock().await;
    new_model.models.push(Model::new(1));
    new_model.models.push(Model::new(701));
    new_model.models.push(Model::new(702));
    new_model.models.push(Model::new(704));
    new_model.models.push(Model::new(705));
    new_model.models.push(Model::new(65535));

    let conf_file = Ini::load_from_file("conf/inverter.ini");

    let mut manufactor: String = String::new();
    let mut inverter_model: String = String::new();
    let mut wmaxrtg: u16 = 0;
    let mut woverextrtg: u16 = 0;
    let mut woverextrtgpf: u16 = 0;
    let mut wundextrtg: u16 = 0;
    let mut wundextrtgpf: u16 = 0;
    let mut vamaxrtg: u16 = 0;
    let mut varmaxinjrtg: u16 = 0;
    let mut varmaxabsrtg: u16 = 0;
    let mut vnomrtg: u16 = 0;
    match conf_file {
        Ok(conf) => {
            for (sec, prop) in conf.iter() {
                if let Some(sec) = sec {
                    if sec.contains("Model1"){
                        for (k, v) in prop.iter() {
                            if k.contains("MANUFACTOR"){
                                manufactor = v.to_string();
                            }else if k.contains("INVERTER_MODEL"){
                                inverter_model = v.to_string();
                            }
                        }
                    }else if sec.contains("Model702"){
                        for (k, v) in prop.iter() {
                            if k.contains("W_MAX_RTG"){
                                println!("Getting W Max Rating");
                                if let Ok(tmp) = v.parse::<u16>() {
                                    println!("W Max Rating: {} W", tmp);
                                    wmaxrtg = tmp;
                                }
                            }else if k.contains("W_OVER_EXT_RTG"){
                                if let Ok(tmp) = v.parse::<u16>() {
                                    woverextrtg = tmp;
                                }
                            }else if k.contains("W_OVER_EXT_RTG_PF"){
                                if let Ok(tmp) = v.parse::<u16>() {
                                    woverextrtgpf = tmp;
                                }
                            }else if k.contains("W_UND_EXT_RTG"){
                                if let Ok(tmp) = v.parse::<u16>() {
                                    wundextrtg = tmp;
                                }
                            }else if k.contains("W_UND_EXT_RTG_PF"){
                                if let Ok(tmp) = v.parse::<u16>() {
                                    wundextrtgpf = tmp;
                                }
                            }else if k.contains("VA_MAX_RTG"){
                                if let Ok(tmp) = v.parse::<u16>() {
                                    vamaxrtg = tmp;
                                }
                            }else if k.contains("VAR_MAX_INJ_RTG"){
                                if let Ok(tmp) = v.parse::<u16>() {
                                    varmaxinjrtg = tmp;
                                }
                            }else if k.contains("VAR_MAX_ABS_RTG"){
                                if let Ok(tmp) = v.parse::<u16>() {
                                    varmaxabsrtg = tmp;
                                }
                            }else if k.contains("V_NOM_RTG"){
                                if let Ok(tmp) = v.parse::<u16>() {
                                    vnomrtg = tmp;
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error in file daemon.ini: {}", e);
            return Err(Box::from(e));
        }
    }

    
    if let Some(idx) = new_model.get_model_index(1) {
        new_model.models[idx].update_data("Mn", &DataTypes::new_string(&manufactor));
        new_model.models[idx].update_data("Md", &DataTypes::new_string(&inverter_model));
    }

    if let Some(idx) = new_model.get_model_index(701) {
        model_701_create_data(&mut new_model.models[idx]);
        new_model.models[idx].update_data("W", &DataTypes::new_i16(10234));
        new_model.models[idx].update_data("TotWhInj", &DataTypes::new_u64(235678));
    }

    if let Some(idx) = new_model.get_model_index(702) {
        model_702_create_data(&mut new_model.models[idx]);
        new_model.models[idx].update_data("WMaxRtg", &DataTypes::new_u16(wmaxrtg));
        new_model.models[idx].update_data("WOvrExtRtg", &DataTypes::new_u16(woverextrtg));
        new_model.models[idx].update_data("WOvrExtRtgPF", &DataTypes::new_u16(woverextrtgpf));
        new_model.models[idx].update_data("WUndExtRtg", &DataTypes::new_u16(wundextrtg));
        new_model.models[idx].update_data("WUndExtRtgPF", &DataTypes::new_u16(wundextrtgpf));
        new_model.models[idx].update_data("VAMaxRtg", &DataTypes::new_u16(vamaxrtg));
        new_model.models[idx].update_data("VarMaxInjRtg", &DataTypes::new_u16(varmaxinjrtg));
        new_model.models[idx].update_data("VarMaxAbsRtg", &DataTypes::new_u16(varmaxabsrtg));
        new_model.models[idx].update_data("VNomRtg", &DataTypes::new_u16(vnomrtg));
    }

    if let Some(idx) = new_model.get_model_index(704) {
        model_704_create_data(&mut new_model.models[idx]);
    }

    if let Some(idx) = new_model.get_model_index(705) {
        model_705_create_data(&mut new_model.models[idx]);
    }

    new_model.compute_addr();
    drop(new_model);

    tokio::select! {
        _ = server_context(socket_addr, Arc::clone(&data)) => unreachable!(),
    }
}

async fn server_context(socket_addr: SocketAddr, data: Arc<Mutex<Models>>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(socket_addr).await?;
    let server = Server::new(listener);

    let data_pointer = &data;
    let on_connected = |stream, _socket_addr| async move {
        let cert_path = Path::new("./pki/serverv3.pem");
        let key_path = Path::new("./pki/serverv3.key");
        let certs = load_certs(cert_path)?;

        let key = load_keys(key_path, None)?;
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        let acceptor = TlsAcceptor::from(Arc::new(config));

        let service = Service::new(Arc::clone(data_pointer));
        let stream = acceptor.accept(stream).await;
        match stream {
            Ok(stream) => {
                println!("Connected: {:?}", stream);
                Ok(Some((service, stream)))
            },
            Err(_) => Ok(None),
        }
        
    };
    let on_process_error = |err| {
        eprintln!("{err}");
    };
    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}

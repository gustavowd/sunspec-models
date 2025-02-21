use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
 
use futures::future;
use sunspec_models::models::{SunspecModels, Model, Models};
use sunspec_models::types::{SunspecTypes, DataTypes};
use tokio::net::TcpListener;
use tokio_modbus::{
    prelude::*,
    server::tcp::{accept_tcp_connection, Server}
};


const SUNSPEC_INIT_ADDR: u16 = 40000;


#[repr(u8)]
#[allow(unused)]
enum Exception {
    IllegalFunction = 0x01,
    IllegalDataAddress = 0x02,
    IllegalDataValue = 0x03,
    ServerDeviceFailure = 0x04,
    Acknowledge = 0x05,
    ServerDeviceBusy = 0x06,
    MemoryParityError = 0x08,
    GatewayPathUnavailable = 0x0A,
    GatewayTargetDevice = 0x0B,
}

struct Service {
    models: Arc<Mutex<Models>>,
}

impl Service {
    fn new(data: Arc<Mutex<Models>>) -> Self {
        Self {
            models: data,
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
        println!("Responding request of unit id: {}", req.slave);
        let res = match req.request.clone() {
            Request::ReadHoldingRegisters(addr, cnt) => {
                let mut start_addr = 0;
                let mut sunspec = self.models.lock().unwrap();
                let mut registers: Vec<u16> = Vec::new();
                if (addr >= SUNSPEC_INIT_ADDR) && (addr < SUNSPEC_INIT_ADDR + 2) {
                    registers = sunspec.id.into();
                    start_addr = SUNSPEC_INIT_ADDR;
                }else {
                    for model in sunspec.models.iter() {
                        if (addr >= model.start_addr) && (addr < model.end_addr) {
                            let model_tmp = model.clone();
                            registers = Vec::<u16>::from(model_tmp);
                            start_addr = model.start_addr;
                        }
                    }
                }
                let end_addr = sunspec.models[sunspec.models.len()-1].end_addr;
                
                // Update current for test purposes
                let a = sunspec.models[1].get_f32("A");
                if let Some(mut a) = a {
                    a += 0.1;
                    sunspec.models[1].update_data("A", &DataTypes::new_f32(a));
                }
                drop(sunspec);

                if addr >= SUNSPEC_INIT_ADDR && (addr + cnt) <= end_addr {
                    println!("Read Holding Registers - Addr: {} - cnt: {}", addr, cnt);
                    let registers = &registers[(addr-start_addr) as usize..(cnt+(addr-start_addr)) as usize];
                    Ok(Response::ReadHoldingRegisters(registers.to_vec()))
                }else {
                    Err(ExceptionCode::IllegalDataAddress)
                }
            },
            Request::WriteMultipleRegisters(addr, regs) => {
                let sunspec = self.models.lock().unwrap();
                let end_addr = sunspec.models[sunspec.models.len()-1].end_addr;
                drop(sunspec);
                if addr >= SUNSPEC_INIT_ADDR && (addr + regs.len() as u16) <= end_addr {
                    println!("Write Multiple Registers - Addr: {} - cnt: {}", addr, regs.len());
                    let mut sunspec = self.models.lock().unwrap();
                    for model in sunspec.models.iter_mut() {
                        if (addr >= model.start_addr) && (addr <= (model.end_addr)) {
                            let regs_vec = regs.to_vec();
                            let model_tmp = Model::from((regs_vec, addr, regs.len() as u16, &model.clone()));
                            *model = model_tmp;
                            println!("{:?}", model);
                        }
                    }
                    drop(sunspec);

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
    let socket_addr = "127.0.0.1:5502".parse().unwrap();

    let data: Arc<Mutex<Models>> = Arc::new(Mutex::new(Models::new()));
    let mut new_model = data.lock().unwrap();
    new_model.models.push(Model::new(1));
    new_model.models.push(Model::new(213));
    new_model.models.push(Model::new(65535));
    
    let idx = new_model.get_model_index(1).unwrap();
    new_model.models[idx].update_data("Mn", &DataTypes::new_string("Manufactor"));
    new_model.models[idx].update_data("Md", &DataTypes::new_string("Model"));
    new_model.models[idx].update_data("Ver", &DataTypes::new_string("ABCD1234"));
    new_model.models[idx].update_data("SN", &DataTypes::new_string("12345678"));

    let idx = new_model.get_model_index(213).unwrap();
    new_model.models[idx].update_data("A", &DataTypes::new_f32(12.5));
    new_model.models[idx].update_data("Hz", &DataTypes::new_f32(60.05));
    new_model.models[idx].update_data("PF", &DataTypes::new_f32(0.92));

    new_model.compute_addr();
    drop(new_model);

    tokio::select! {
        _ = server_context(socket_addr, Arc::clone(&data)) => unreachable!(),
    }
}

async fn server_context(socket_addr: SocketAddr, data: Arc<Mutex<Models>>) -> anyhow::Result<()> {
    println!("Starting up server on {socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    let server = Server::new(listener);
    let new_service = |_socket_addr| Ok(Some(Service::new(Arc::clone(&data))));
    let on_connected = |stream, socket_addr| async move {
        accept_tcp_connection(stream, socket_addr, new_service)
    };
    let on_process_error = |err| {
        eprintln!("{err}");
    };
    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}

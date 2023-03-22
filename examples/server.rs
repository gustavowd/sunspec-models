use std::{
    mem,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
 
use futures::future;
use sunspec_models::models::{Models, SunspecID, Model1, ModelEnd};
use sunspec_models::models::models200::Model213;
use tokio::net::{TcpListener};
use tokio_modbus::{
    prelude::*,
    server::tcp::{accept_tcp_connection, Server},
};


const SUNSPEC_INIT_ADDR: u16 = 40000;

fn error_response(req: &Request, exc: Exception) -> std::io::Result<Response> {
    Ok(Response::Custom(req2functioncode(&req) | 0x80, vec![exc as u8]))
}

fn req2functioncode(req: &Request) -> u8 {
    match *req {
        Request::ReadCoils(_, _) => 0x01,
        Request::ReadDiscreteInputs(_, _) => 0x02,
        Request::WriteSingleCoil(_, _) => 0x05,
        Request::WriteMultipleCoils(_, _) => 0x0F,
        Request::ReadInputRegisters(_, _) => 0x04,
        Request::ReadHoldingRegisters(_, _) => 0x03,
        Request::WriteSingleRegister(_, _) => 0x06,
        Request::WriteMultipleRegisters(_, _) => 0x10,
        Request::ReadWriteMultipleRegisters(_, _, _, _) => 0x17,
        Request::Custom(code, _) => code,
        _ => 0x00,
    }
}

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

#[derive(Debug, Clone)]
pub struct Sunspec {
    id: SunspecID,
    model1: Model1,
    model213: Model213,
    model_end: ModelEnd,
}

impl Sunspec {
    fn new () -> Sunspec {
        Sunspec {
            id: SunspecID::new(),
            model1: Model1::new(),
            model213: Model213::new(),
            model_end: ModelEnd::new()
        }
    }
}

struct Service {
    models: Arc<Mutex<Sunspec>>,
}

impl Service {
    fn new(data: Arc<Mutex<Sunspec>>) -> Self {
        Self {
            models: data,
        }
    }
}

impl tokio_modbus::server::Service for Service {
    type Request = SlaveRequest;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future 
    {
        println!("Responding request of unit id: {}", req.slave);
        match req.request.clone() {
            Request::ReadHoldingRegisters(addr, cnt) => {
                let mut models = self.models.lock().unwrap();

                let mut registers: Vec<u16> = models.id.into();
                let model1a = models.model1.clone();
                registers.extend(Vec::<u16>::from(model1a));
                registers.extend(Vec::<u16>::from(models.model213));
                registers.extend(Vec::<u16>::from(models.model_end));
                // Emulate current variation
                models.model213.a += 0.1;
                drop(models);

                if addr >= SUNSPEC_INIT_ADDR && (addr + cnt) <= (SUNSPEC_INIT_ADDR + (mem::size_of::<Sunspec>() / 2) as u16) {
                    println!("Read Holding Registers - Addr: {} - cnt: {}", addr, cnt);
                    let registers = &registers[(addr - SUNSPEC_INIT_ADDR) as usize..(cnt + (addr - SUNSPEC_INIT_ADDR)) as usize];
                    future::ready(Ok(Response::ReadHoldingRegisters(registers.to_vec())))
                }else {
                    future::ready(error_response(&req.request, Exception::IllegalDataAddress))
                }
            },
            Request::WriteMultipleRegisters(addr, regs) => {
                if addr >= SUNSPEC_INIT_ADDR && (addr + regs.len() as u16) <= (SUNSPEC_INIT_ADDR + (mem::size_of::<Sunspec>() / 2) as u16) {
                    println!("Write Multiple Registers - Addr: {} - cnt: {}", addr, regs.len());
                    let mut models = self.models.lock().unwrap();
                    if (addr >= models.model1.start_addr) && (addr <= (models.model1.start_addr + models.model1.qtd)) {
                        let model1_tmp = Model1::from((regs.clone(), addr, &models.model1));
                        models.model1 = model1_tmp;
                    }
                    drop(models);

                    future::ready(Ok(Response::WriteMultipleRegisters(addr,regs.len() as u16)))
                }else {
                    future::ready(error_response(&req.request, Exception::IllegalDataAddress))
                }
            },
            _ => future::ready(error_response(&req.request, Exception::IllegalFunction))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "127.0.0.1:5502".parse().unwrap();

    let data: Arc<Mutex<Sunspec>> = Arc::new(Mutex::new(Sunspec::new()));
    let mut new_model = data.lock().unwrap();
    new_model.model1.start_addr = SUNSPEC_INIT_ADDR + 2;
    new_model.model1.manufacturer.value = "Manufactor".to_string();
    new_model.model1.model.value = "Model".to_string();
    new_model.model1.serial_number.value = "ABCD1234".to_string();
    new_model.model1.options.value = "Options".to_string();

    new_model.model213.start_addr = SUNSPEC_INIT_ADDR + 2 + new_model.model213.qtd + 2;
    new_model.model213.a = 12.5;
    new_model.model213.hz = 60.05;
    new_model.model213.pf = 0.92;
    drop(new_model);

    tokio::select! {
        _ = server_context(socket_addr, Arc::clone(&data)) => unreachable!(),
    }
}

async fn server_context(socket_addr: SocketAddr, data: Arc<Mutex<Sunspec>>) -> anyhow::Result<()> {
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

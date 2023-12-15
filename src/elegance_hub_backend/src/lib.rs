#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};


type Memory = VirtualMemory<DefaultMemoryImpl>; 
type IdCell = Cell<u64, Memory>;


// Struct to represent a Client
#[derive (candid::CandidType, Clone,Serialize, Deserialize)]
struct Client {
    id: u64,
    name: String,
    email: String,
    phone: String,
    address: String,
}

// Struct to represent a Service offered by the beauty parlour
#[derive (candid::CandidType, Clone,Serialize, Deserialize)]
struct Service {
    id: u64,
    name: String,
    description: String,
    duration: u64,
    price: u64,
}

// Struct to represent an Appointment
#[derive (candid::CandidType, Clone,Serialize, Deserialize)]
struct Appointment {
    id: u64,
    client_id: u64,
    service_id: u64,
    date: String, // format: dd/mm/yyyy e.g 01/01/2021
    time: String,
    status: String,
    created_at:u64,
    updated_at:Option<u64>
}

// Implement the Storable and BoundedStorable traits for the Client struct
impl Storable for Client {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
      Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
      Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl  BoundedStorable for Client {
   const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}

// Implement the Storable and BoundedStorable traits for the Service struct
impl Storable for Service {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
      Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
      Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl  BoundedStorable for Service {
   const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}

// Implement the Storable and BoundedStorable traits for the Appointment struct
impl Storable for Appointment {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
      Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
      Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl  BoundedStorable for Appointment {
   const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static CLIENT_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );
    static SERVICE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );
    static APPOINTMENT_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0)
            .expect("Cannot create a counter")
    );
    static CLIENT_STORAGE: RefCell<StableBTreeMap<u64, Client, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static SERVICE_STORAGE: RefCell<StableBTreeMap<u64, Service, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    static APPOINTMENT_STORAGE: RefCell<StableBTreeMap<u64, Appointment, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );
}


// client payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct ClientPayload {
    name: String,
    email: String,
    phone: String,
    address: String,
}

// service payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct ServicePayload {
    name: String,
    description: String,
    duration: u64,
    price: u64,
}

// appointment payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct AppointmentPayload {
    client_id: u64,
    service_id: u64,
    date: String, // format: dd/mm/yyyy e.g 01/01/2021
    time: String,
    status: String,
}

// Client CRUD

// create client
#[ic_cdk::update]
fn create_client(client_payload: ClientPayload) -> Result<Client, String> {
    let id = CLIENT_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    // validate client payload one liner
    if client_payload.name.is_empty() || client_payload.email.is_empty() || client_payload.phone.is_empty() || client_payload.address.is_empty() {
        return Err("Invalid client payload, Fill in the fields".to_string());
    }

    let client = Client {
        id,
        name: client_payload.name,
        email: client_payload.email,
        phone: client_payload.phone,
        address: client_payload.address,
    };
    do_insert_client(&client);
    Ok(client)
}

// update client 
#[ic_cdk::update]
fn update_client(client_id: u64, client_payload: ClientPayload) -> Result<Client, String> {
    let client = CLIENT_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&client_id)
            .ok_or_else(|| "Client not found".to_string())
    })?;
    let updated_client = Client {
        id: client.id,
        name: client_payload.name,
        email: client_payload.email,
        phone: client_payload.phone,
        address: client_payload.address,
    };
    do_insert_client(&updated_client);
    Ok(updated_client)
}

// A helper method to perform inserting of a client .
fn do_insert_client(client: &Client) {
    CLIENT_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(client.id, client.clone())
    });
}

// get client by id
#[ic_cdk::query]
fn get_client_by_id(client_id: u64) -> Result<Client, String> {
    CLIENT_STORAGE.with(|service| {
        service
            .borrow()
            .get(&client_id)
            .ok_or_else(|| "Client not found".to_string())
    })
}

// get all clients
#[ic_cdk::query]
fn get_all_clients() -> Vec<Client> {
    CLIENT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, client)| client.clone())
            .collect()
    })
}

// delete client by id
#[ic_cdk::update]
fn delete_client_by_id(client_id: u64) -> Result<(), String> {
    CLIENT_STORAGE.with(|service| {
        service
            .borrow_mut()
            .remove(&client_id)
            .ok_or_else(|| "Client not found".to_string())
    })?;
    Ok(())
}

// Service CRUD

// create service
#[ic_cdk::update]
fn create_service(service_payload: ServicePayload) -> Result<Service, String> {
    let id = SERVICE_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    // validate service payload one liner
    if service_payload.name.is_empty() || service_payload.description.is_empty() || service_payload.duration == 0 || service_payload.price == 0 {
        return Err("Invalid service payload, Fill in the fields".to_string());
    }

    let service = Service {
        id,
        name: service_payload.name,
        description: service_payload.description,
        duration: service_payload.duration,
        price: service_payload.price,
    };
    do_insert_service(&service);
    Ok(service)
}

// update service
#[ic_cdk::update]
fn update_service(service_id: u64, service_payload: ServicePayload) -> Result<Service, String> {
    let service = SERVICE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&service_id)
            .ok_or_else(|| "Service not found".to_string())
    })?;
    let updated_service = Service {
        id: service.id,
        name: service_payload.name,
        description: service_payload.description,
        duration: service_payload.duration,
        price: service_payload.price,
    };
    do_insert_service(&updated_service);
    Ok(updated_service)
}

// A helper method to perform inserting of a service .
fn do_insert_service(service: &Service) {
    SERVICE_STORAGE.with(|s| {
        s
            .borrow_mut()
            .insert(service.id, service.clone())
    });
}

// get service by id
#[ic_cdk::query]
fn get_service_by_id(service_id: u64) -> Result<Service, String> {
    SERVICE_STORAGE.with(|service| {
        service
            .borrow()
            .get(&service_id)
            .ok_or_else(|| "Service not found".to_string())
    })
}

// get all services
#[ic_cdk::query]
fn get_all_services() -> Vec<Service> {
    SERVICE_STORAGE.with(|s| {
        s
            .borrow()
            .iter()
            .map(|(_, service)| service.clone())
            .collect()
    })
}

// delete service by id
#[ic_cdk::update]
fn delete_service_by_id(service_id: u64) -> Result<(), String> {
    SERVICE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .remove(&service_id)
            .ok_or_else(|| "Service not found".to_string())
    })?;
    Ok(())
}

// Appointment CRUD

// create appointment
#[ic_cdk::update]
fn create_appointment(appointment_payload: AppointmentPayload) -> Result<Appointment, String> {
    let id = APPOINTMENT_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    // validate appointment payload one liner
    if appointment_payload.date.is_empty() || appointment_payload.time.is_empty() || appointment_payload.status.is_empty() {
        return Err("Invalid appointment payload, Fill in the fields".to_string());
    }

    let appointment = Appointment {
        id,
        client_id: appointment_payload.client_id,
        service_id: appointment_payload.service_id,
        date: appointment_payload.date,
        time: appointment_payload.time,
        status: appointment_payload.status,
        created_at: time(),
        updated_at: None,
    };
    do_insert_appointment(&appointment);
    Ok(appointment)
}

// update appointment
#[ic_cdk::update]
fn update_appointment(appointment_id: u64, appointment_payload: AppointmentPayload) -> Result<Appointment, String> {
    let appointment = APPOINTMENT_STORAGE.with(|appointment| {
        appointment
            .borrow_mut()
            .get(&appointment_id)
            .ok_or_else(|| "Appointment not found".to_string())
    })?;
    let updated_appointment = Appointment {
        id: appointment.id,
        client_id: appointment_payload.client_id,
        service_id: appointment_payload.service_id,
        date: appointment_payload.date,
        time: appointment_payload.time,
        status: appointment_payload.status,
        created_at: appointment.created_at,
        updated_at: Some(time()),
    };
    do_insert_appointment(&updated_appointment);
    Ok(updated_appointment)
}

// A helper method to perform inserting of an appointment .
fn do_insert_appointment(appointment: &Appointment) {
    APPOINTMENT_STORAGE.with(|s| {
        s
            .borrow_mut()
            .insert(appointment.id, appointment.clone())
    });
}

// get appointment by id
#[ic_cdk::query]
fn get_appointment_by_id(appointment_id: u64) -> Result<Appointment, String> {
    APPOINTMENT_STORAGE.with(|s| {
        s
            .borrow()
            .get(&appointment_id)
            .ok_or_else(|| "Appointment not found".to_string())
    })
}

// get all appointments
#[ic_cdk::query]
fn get_all_appointments() -> Vec<Appointment> {
    APPOINTMENT_STORAGE.with(|s| {
        s
            .borrow()
            .iter()
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

// delete appointment by id
#[ic_cdk::update]
fn delete_appointment_by_id(appointment_id: u64) -> Result<(), String> {
    APPOINTMENT_STORAGE.with(|s| {
        s
            .borrow_mut()
            .remove(&appointment_id)
            .ok_or_else(|| "Appointment not found".to_string())
    })?;
    Ok(())
}

// get all appointments by client id
#[ic_cdk::query]
fn get_all_appointments_by_client_id(client_id: u64) -> Vec<Appointment> {
    APPOINTMENT_STORAGE.with(|s| {
        s
            .borrow()
            .iter()
            .filter(|(_, appointment)| appointment.client_id == client_id)
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

// get all appointments by service id
#[ic_cdk::query]
fn get_all_appointments_by_service_id(service_id: u64) -> Vec<Appointment> {
    APPOINTMENT_STORAGE.with(|s| {
        s
            .borrow()
            .iter()
            .filter(|(_, appointment)| appointment.service_id == service_id)
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

// get all appointments by date
#[ic_cdk::query]
fn get_all_appointments_by_date(date: String) -> Vec<Appointment> {
    APPOINTMENT_STORAGE.with(|s| {
        s
            .borrow()
            .iter()
            .filter(|(_, appointment)| appointment.date == date)
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

// get all appointments by status
#[ic_cdk::query]
fn get_all_appointments_by_status(status: String) -> Vec<Appointment> {
    APPOINTMENT_STORAGE.with(|s| {
        s
            .borrow()
            .iter()
            .filter(|(_, appointment)| appointment.status.to_lowercase() == status.to_lowercase())
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

// Calculate total revenue generated by a service within a specific date 
#[ic_cdk::query]
fn get_total_revenue_by_service_id_and_date(service_id: u64, date: String) -> u64 {
    let appointments = APPOINTMENT_STORAGE.with(|appointment| {
        appointment
            .borrow()
            .iter()
            .filter(|(_, appointment)| appointment.service_id == service_id && appointment.date == date)
            .map(|(_, appointment)| appointment.clone())
            .collect::<Vec<Appointment>>()
    });
    let mut total_revenue = 0;
    for appointment in appointments {
        total_revenue += SERVICE_STORAGE.with(|s| {
            s
                .borrow()
                .get(&appointment.service_id)
                .ok_or_else(|| "Service not found".to_string())
        }).unwrap().price;
    }
    total_revenue
}


// fununction to get most popular service
#[ic_cdk::query]
fn get_most_popular_service() -> Service {
    let appointments = APPOINTMENT_STORAGE.with(|appointment| {
        appointment
            .borrow()
            .iter()
            .map(|(_, appointment)| appointment.clone())
            .collect::<Vec<Appointment>>()
    });
    let mut service_id_count_map = std::collections::HashMap::new();
    for appointment in appointments {
        let count = service_id_count_map.entry(appointment.service_id).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut max_service_id = 0;
    for (service_id, count) in service_id_count_map {
        if count > max_count {
            max_count = count;
            max_service_id = service_id;
        }
    }
    SERVICE_STORAGE.with(|s| {
        s
            .borrow()
            .get(&max_service_id)
            .ok_or_else(|| "Service not found".to_string())
    }).unwrap().clone()
}

// fununction to get most popular client
#[ic_cdk::query]
fn get_most_popular_client() -> Client {
    let appointments = APPOINTMENT_STORAGE.with(|s| {
        s
            .borrow()
            .iter()
            .map(|(_, appointment)| appointment.clone())
            .collect::<Vec<Appointment>>()
    });
    let mut client_id_count_map = std::collections::HashMap::new();
    for appointment in appointments {
        let count = client_id_count_map.entry(appointment.client_id).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut max_client_id = 0;
    for (client_id, count) in client_id_count_map {
        if count > max_count {
            max_count = count;
            max_client_id = client_id;
        }
    }
    CLIENT_STORAGE.with(|client| {
        client
            .borrow()
            .get(&max_client_id)
            .ok_or_else(|| "Client not found".to_string())
    }).unwrap().clone()
}

// Export the candid interface
ic_cdk::export_candid!();
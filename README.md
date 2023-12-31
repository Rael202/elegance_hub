# Elegance Hub

Welcome to your new elegance_hub project and to the internet computer development community. 

The goal of this project is to provide a backend system for managing a elegance hub operations, including handling clients, services offered, and appointments. The project utilizes the Rust programming language and leverages the Internet Computer's capabilities for secure and decentralized applications.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

## Features 
- **Client Management**: Create, update, delete, and retrieve client details such as name, email, phone, and address.
- **Service Management**: CRUD operations for services, including name, description, duration, and price.
- **Appointment Handling**: Create, update, delete appointments with details like client ID, service ID, date, time, and status.
- **Business Analytics**: Functions to analyze revenue generated by services, determine the most popular services, and identify the most frequent clients.

## Functionality of each function
### Create Client 
    fn create_client(client_payload: ClientPayload) -> Result<Client, String> 
Description:

Adds a new client to the beauty parlour's database with details such as name, email, phone, and address. Returns the newly created client.

### Update Client 
    fn update_client(client_id: u64, client_payload: ClientPayload) -> Result<Client, String>
Description:

Updates existing client information in the storage by providing the client's ID and updated details. Returns the updated client information.


### Get Client by ID:
    fn get_client_by_id(client_id: u64) -> Result<Client, String> 

Description:

Retrieves specific client information by providing the client's ID.

### Get all Clients 
    fn get_all_clients() -> Vec<Client>
Description:

Retrieves a list of all clients stored.


### Delete Client by ID
    fn delete_client_by_id(client_id: u64) -> Result<(), String>
Description:

Deletes a client from the database by providing the client's ID.

### service Management 

### Create Service
    fn create_service(service_payload: ServicePayload) -> Result<Service, String>
Description:

Adds a new service offered by the beauty parlour to the database with details such as name, description, duration, and price. Returns the newly created service.

### Update Service 
    fn update_service(service_id: u64, service_payload: ServicePayload) -> Result<Service, String> 
Description:

Updates existing service information in the database by providing the service's ID and updated details. Returns the updated service information.

### Get Service by ID
    fn get_service_by_id(service_id: u64) -> Result<Service, String> 

Description:

Retrieves specific service information by providing the service's ID.

### Get All services
    fn get_all_services() -> Vec<Service> 
Description:

Retrieves a list of all services

### Delete service by ID
    fn delete_service_by_id(service_id: u64) -> Result<(), String> 
Description:

Deletes a service from the database by providing the service's ID.


### Appointment 

### Create Appointment 
    fn create_appointment(appointment_payload: AppointmentPayload) -> Result<Appointment, String>
Description:

Creates a new appointment for a client with a specific service.Returns the newly created appointment.
Note:date take using format: dd/mm/yyyy e.g 01/01/2021
### Update Appointment 
    fn update_appointment(appointment_id: u64, appointment_payload: AppointmentPayload) -> Result<Appointment, String>
Description:

Updates existing appointment information in the database by providing the appointment's ID and updated details. Returns the updated appointment information.

### Get Appointment by Id 

    fn get_appointment_by_id(appointment_id: u64) -> Result<Appointment, String>
Description:

Retrieves specific appointment information by providing the appointment's ID.

### Get all Appointments 
    fn get_all_appointments() -> Vec<Appointment> 

Description:

Retrieves a list of all appointments booked

### Delete Appointments by ID   
    fn delete_appointment_by_id(appointment_id: u64) -> Result<(), String>

Description:

Deletes an appointment from the database by providing the appointment's ID.


### Get all Appointments by Client ID

    fn get_all_appointments_by_client_id(client_id: u64) -> Vec<Appointment>

Description:

Retrieves a list of all appointments made by a specific client, identified by their ID.

### Get all Appointments by Service ID 
    fn get_all_appointments_by_service_id(service_id: u64) -> Vec<Appointment>

Description:

Retrieves a list of all appointments booked for a specific service, identified by its ID.

### Get all Appointments by Date 
    fn get_all_appointments_by_date(date: String) -> Vec<Appointment> 

Description:

Retrieves a list of all appointments scheduled for a particular date.

### Get all Appointments by status 
    fn get_all_appointments_by_status(status: String) -> Vec<Appointment>
Description:

Retrieves a list of all appointments based on a specified status (e.g., confirmed, pending, completed).

### Get Total Revenue by Service ID and Date 
    fn get_total_revenue_by_service_id_and_date(service_id: u64, date: String) -> u64
Description:

Calculates the total revenue generated by a specific service on a particular date based on appointments made.

### Get most popular service 
    fn get_most_popular_service() -> Service
Description:

Retrieves information about the service that has been booked the most, making it the most popular service in terms of appointments.

### Get most popular client 
    fn get_most_popular_client() -> Client
Description:

Retrieves information about the client who has booked the most appointments, making them the most popular client based on the number of appointments.














To learn more before you start working with elegance_hub, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd elegance_hub/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor

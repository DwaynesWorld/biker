# Biker

## Data Model

### users
- id: int64
- first_name: string
- last_name: string
- email: string
- phone: string
- created_at: datetime
- modified_at: datetime

### bikes
- id: int64
- code: string
- model: string
- location: point
- created_at: datetime
- modified_at: datetime

### payments
- id: int64
- ride_id: int64
- status: enum [pending | cancelled | rejected | completed]
- amount: int32
- method: enum [credit_card | debit_card | apple_pay | paypal]
- created_at: datetime
- modified_at: datetime
- 
### rides
- user_id: int64
- bike_id: int64
- status: enum [reserved | in-progress | completed | cancelled]
- source: point
- destination: point
- created_at: datetime
- modified_at: datetime

## API

- POST /users/register
- GET  /users/:id
- GET  /bikes/find
- POST /bikes
- PUT  /bikes/:id
- POST /rides

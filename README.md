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


### Data (Lat, Long)

#### Users
```sql
INSERT INTO "public"."users" (
    "id", 
    "name", 
    "nickname", 
    "email", 
    "email_verified", 
    "phone", 
    "picture_uri", 
    "status", 
    "default_payment_method", 
    "created_at", 
    "updated_at"
) 
VALUES (
    'auth0|66562ec12d6ea328db8e223b', 
    'Kyle Thompson', 
    'kt', 
    'k@t.com', 
    'f', 
    '8322348004', 
    'https://s.gravatar.com/avatar/7e15c45559ac78456d561b7390a9704b?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fk.png', 
    'ACTIVE', 
    'APPLE_PAY', 
    '2024-05-28 21:22:00.269745+00', 
    '2024-05-28 21:22:00.269745+00'
);
```

#### Bikes
- Current Location (1431 Cafe): 30.523480388158305,-97.82317842925178
- Restaurant: 30.524585270122124,-97.82005549355522
- Hospital: 30.52377047409439,-97.81929777081675
- Park: 30.533159937986014,-97.77605868419684
- Downtown: 30.265310134209358,-97.74916435969541

```sql
INSERT INTO "public"."bikes" ("id", "code", "model", "location", "created_at", "updated_at") VALUES ('018fc03d-19c8-7306-982b-0808701dd9ff', 'bike-1', 'OEM Spec 1', 'SRID=4326;POINT(-97.82005549355522 30.524585270122124)', now(), now());
INSERT INTO "public"."bikes" ("id", "code", "model", "location", "created_at", "updated_at") VALUES ('018fc03d-19c8-7bbd-af03-83e9cac4698a', 'bike-2', 'OEM Spec 1', 'SRID=4326;POINT(-97.81929777081675 30.52377047409439)',  now(), now());
INSERT INTO "public"."bikes" ("id", "code", "model", "location", "created_at", "updated_at") VALUES ('018fc03d-19c8-782a-9cd6-8f2e3e9cff20', 'bike-3', 'OEM Spec 1', 'SRID=4326;POINT(-97.77605868419684 30.533159937986014)', now(), now());
INSERT INTO "public"."bikes" ("id", "code", "model", "location", "created_at", "updated_at") VALUES ('018fc03d-19c8-7bd7-9941-0756143974f2', 'bike-4', 'OEM Spec 1', 'SRID=4326;POINT(-97.74916435969541 30.265310134209358)', now(), now());
```

#### Nearest Bikes (degrees)
```sql
select *, b. "location" <-> ST_POINT (-97.82317842925178, 30.523480388158305, 4326) as dist
from bikes b
order by dist asc;
```

#### Nearest Bikes (meters)
```sql
select *, st_distance(geography (b. "location"), ST_POINT (-97.82317842925178, 30.523480388158305, 4326)) as dist
from bikes b
order by dist asc;
```

#### Nearest Bikes within 1000 meters
```sql
select *, st_distance(b."location"::geography, st_point(- 97.82317842925178, 30.523480388158305, 4326)::geography) as dist
from bikes b
where st_dwithin(b."location"::geography, st_point(- 97.82317842925178, 30.523480388158305, 4326)::geography, 1000)
order by dist asc;
```

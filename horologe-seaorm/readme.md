```sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    scheduled_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    status VARCHAR NOT NULL CHECK (status IN ('Pending', 'Processing', 'Completed', 'Failed'))
);
```
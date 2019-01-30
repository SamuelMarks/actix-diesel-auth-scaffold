CREATE TABLE user_tbl (
    title text,
    email text,
    password text,
    roles text DEFAULT 'registered;'::text,
    "createdAt" timestamp with time zone,
    "updatedAt" timestamp with time zone,
    PRIMARY KEY(email)
);

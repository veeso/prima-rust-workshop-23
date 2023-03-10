DO $$ BEGIN
  CREATE TYPE order_status AS ENUM (
      'created',
      'preparing',
      'payment_refused',
      'shipped'
  );
  EXCEPTION
      WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS customer (
  id uuid NOT NULL PRIMARY KEY,
  email text NOT NULL UNIQUE,
  password text NOT NULL,
  created_at timestamp
);

CREATE TABLE IF NOT EXISTS article (
  id uuid NOT NULL PRIMARY KEY,
  name text NOT NULL,
  description text NOT NULL,
  unit_price decimal NOT NULL
);

CREATE TABLE IF NOT EXISTS customer_order (
  id uuid NOT NULL PRIMARY KEY,
  customer_id uuid REFERENCES customer(id) ON DELETE RESTRICT,
  created_at timestamp,
  status order_status NOT NULL,
  transaction_id uuid
);

CREATE TABLE IF NOT EXISTS order_article (
  id uuid NOT NULL PRIMARY KEY,
  quantity integer NOT NULL,
  unit_price decimal NOT NULL,
  order_id uuid REFERENCES customer_order(id) ON DELETE RESTRICT,
  article_id uuid REFERENCES article(id) ON DELETE RESTRICT
);

-- Add migration script here
CREATE TABLE "categories" (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  user_id BIGINT NOT NULL,
  description TEXT,  
  balance DECIMAL(15, 2) DEFAULT 0.00 NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_user
    FOREIGN KEY(user_id) 
    REFERENCES "users"(id)
    ON DELETE CASCADE
);

-- Create an index on user_id for faster joins
CREATE INDEX categories_user_id_idx ON "categories" (user_id);

-- Create a trigger to call the function
CREATE TRIGGER update_categories_modtime
BEFORE UPDATE ON "categories"
FOR EACH ROW
EXECUTE FUNCTION update_modified_column();
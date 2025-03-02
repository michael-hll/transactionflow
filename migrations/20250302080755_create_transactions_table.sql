-- Add migration script here
CREATE TABLE "transactions" (
  id BIGSERIAL PRIMARY KEY,  
  user_id BIGINT NOT NULL,
  category_id BIGINT NOT NULL,
  type VARCHAR(255) NOT NULL,
  amount DECIMAL(15, 2) DEFAULT 0.00 NOT NULL,
  memo TEXT NOT NULL,
  description TEXT,    
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_user
    FOREIGN KEY(user_id) 
    REFERENCES "users"(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_category
    FOREIGN KEY(category_id) 
    REFERENCES "categories"(id)
    ON DELETE CASCADE
);

-- Create an index on user_id for faster joins
CREATE INDEX transactions_user_id_idx ON "transactions" (user_id);
CREATE INDEX transactions_category_id_idx ON "transactions" (category_id);

-- Create a trigger to call the function
CREATE TRIGGER update_transactions_modtime
BEFORE UPDATE ON "transactions"
FOR EACH ROW
EXECUTE FUNCTION update_modified_column();
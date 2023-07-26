ALTER TABLE sessions 
ADD CONSTRAINT fk_sessions_user_id_to_user_id 
FOREIGN KEY (user_id) 
REFERENCES users (user_id)
ON DELETE CASCADE;

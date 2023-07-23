ALTER TABLE characters 
ADD CONSTRAINT fk_character_id_to_user_id 
FOREIGN KEY (character_id) 
REFERENCES users (user_id)
ON DELETE CASCADE;

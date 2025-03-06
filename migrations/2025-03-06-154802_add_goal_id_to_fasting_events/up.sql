ALTER TABLE fasting_events ADD COLUMN goal_id INTEGER NULL;
ALTER TABLE fasting_events ADD CONSTRAINT fk_goal FOREIGN KEY (goal_id) REFERENCES fasting_goals(id) ON DELETE SET NULL;

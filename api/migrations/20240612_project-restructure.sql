-- Rename mods to projects
ALTER TABLE mods
RENAME TO projects;

-- Allow projects to hold both Campaigns and Mods
ALTER TABLE projects
ADD COLUMN project_type VARCHAR(32);

ALTER TABLE projects
ADD CONSTRAINT project_type_check
CHECK (project_type IN ('campaign', 'addon'));

-- Change how project owners work
ALTER TABLE projects
DROP COLUMN owner;

CREATE TABLE project_owners (
	project_id INT REFERENCES projects(id),
	user_id INT REFERENCES users(id),
	PRIMARY KEY (project_id, user_id)
);

-- Add project tags
ALTER TABLE projects
ADD COLUMN project_tags VARCHAR(32)[];

-- Create servers table
CREATE TABLE servers (
    id SERIAL PRIMARY KEY,
    server_name VARCHAR(255) NOT NULL,
    ip_address INET NOT NULL,
	server_tags VARCHAR(32)[],
	player_limit integer
);

CREATE TABLE server_owners (
	server_id INT REFERENCES servers(id),
	user_id INT REFERENCES users(id),
	PRIMARY KEY (server_id, user_id)
);

-- WebVitals Telemetry storage for website and launcher
CREATE TABLE IF NOT EXISTS webvitals (
	cls FLOAT NOT NULL,
	fcp FLOAT NOT NULL,
	fid FLOAT NOT NULL,
	inp FLOAT NOT NULL,
	lcp FLOAT NOT NULL,
	ttfb FLOAT NOT NULL,
	user_agent TEXT NOT NULL
);
-- Activity Telemetry storage for historical playtime data
CREATE TABLE IF NOT EXISTS activity (
	ping_reason TEXT NOT NULL,
	client_type TEXT NOT NULL,
	uptime FLOAT NOT NULL,
	player_count INTEGER NOT NULL,
	mod_count INTEGER NOT NULL
)

-- Change the ID of users to the Username instead
ALTER TABLE users
DROP CONSTRAINT users_pkey CASCADE;

ALTER TABLE users
ADD PRIMARY KEY (username);

-- Add a Creation Date param
ALTER TABLE users
ADD COLUMN creation_datetime timestamptz default CURRENT_TIMESTAMP NOT NULL

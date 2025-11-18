-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at DESC);

-- Insert some sample data
INSERT INTO users (id, name, email, created_at) VALUES
    ('550e8400-e29b-41d4-a716-446655440000', 'Alice', 'alice@example.com', NOW()),
    ('550e8400-e29b-41d4-a716-446655440001', 'Bob', 'bob@example.com', NOW()),
    ('550e8400-e29b-41d4-a716-446655440002', 'Charlie', 'charlie@example.com', NOW())
ON CONFLICT (email) DO NOTHING;


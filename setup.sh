#/bin/bash

echo "== Setting up address book project ==="

echo "Creating database..."
#sudo -u postgres psql -c "CREATE DATABASE IF NOT EXIST address_db;"
sudo -u psql -U postgres -tc "SELECT 1 FROM pg_database WHERE datname='address_db'" | grep -q 1 ||
sudo -u psql -U postgres -c "CREATE DATABASE address_db"





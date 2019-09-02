export ROCKET_DATABASES="{mysql={url=\"mysql://$MARIADB_USER:$MARIADB_PASSWORD@$MARIADB_HOSTNAME/$MARIADB_DATABASE\"}}"
export DATABASE_URL="mysql://$MARIADB_USER:$MARIADB_PASSWORD@$MARIADB_HOSTNAME/$MARIADB_DATABASE"
export RUST_LOG=info

diesel migration run

cargo run --release

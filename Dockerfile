# Use the official PostgreSQL image
FROM postgres:13

# Set environment variables
ENV POSTGRES_DB=taskmanager
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=password

# Copy initialization scripts
COPY init.sql /docker-entrypoint-initdb.d/

# Expose the PostgreSQL port
EXPOSE 5432

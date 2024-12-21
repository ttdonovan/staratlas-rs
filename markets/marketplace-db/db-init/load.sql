COPY certificate_mints FROM 'db-init/certificate_mints.csv' (FORMAT 'csv', header 1, delimiter ',', quote '"');
COPY galactic_mints FROM 'db-init/galactic_mints.csv' (FORMAT 'csv', header 1, delimiter ',', quote '"');
COPY orders FROM 'db-init/orders.csv' (FORMAT 'csv', header 1, delimiter ',', quote '"');

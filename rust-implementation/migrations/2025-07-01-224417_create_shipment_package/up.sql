CREATE TABLE shipment_packages (
    package_id TEXT NOT NULL PRIMARY KEY,
    -- NOTE: we could add an index to shipment_id.
    shipment_id INTEGER NOT NULL,

    FOREIGN KEY (shipment_id) REFERENCES shipments (shipment_id)
);

-- Create Subscriptions Table
CREATE TABLE Subscriptions(
    Id UUID NOT NULL,
    PRIMARY KEY (Id),
    Email TEXT NOT NULL UNIQUE,
    Name TEXT NOT NULL,
    SubscribedAt TIMESTAMPTZ NOT NULL
);

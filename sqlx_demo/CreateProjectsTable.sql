-- Create Projects Table
CREATE TABLE Projects (
    ProjectId UUID PRIMARY KEY,
    ProjectName VARCHAR(100) NOT NULL,
    ProjectStartDate DATE NOT NULL,
    ProjectEndDate DATE NOT NULL,
    PayRate DECIMAL(10, 2) NOT NULL,
    ProjectTotalDuration TIME NOT NULL DEFAULT '00:00:00',
    ProjectTotalPay DECIMAL(10, 2) NOT NULL DEFAULT '0.00'
);
-- Create Tasks Table
CREATE TABLE ProjectTasks (
    TaskId UUID PRIMARY KEY,
    ProjectId UUID NOT NULL,
    TaskName VARCHAR(100) NOT NULL,
    TaskTotalDuration TIME NOT NULL DEFAULT '00:00:00',
    FOREIGN KEY (ProjectId) REFERENCES Projects(ProjectId) ON DELETE CASCADE
);
-- Create TaskTimings Table
CREATE TABLE TaskTimings (
    TimingId SERIAL UNIQUE NOT NULL,
    TaskId UUID NOT NULL,
    StartTimestamp TIMESTAMP NOT NULL,
    EndTimestamp TIMESTAMP NOT NULL,
    PRIMARY KEY (TimingId),
    FOREIGN KEY (TaskId) REFERENCES ProjectTasks(TaskId) ON DELETE CASCADE
);

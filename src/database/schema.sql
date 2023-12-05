
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'MonitorResultType') THEN
        CREATE TYPE "MonitorResultType" AS ENUM (
            'SUCCESS',
            'FAIL',
            'ERROR',
        );
    END IF;
END
$$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'MonitorType') THEN
        CREATE TYPE "MonitorType" AS ENUM (
            'ENDPOINT_SECURITY',
            'POLICY_EVALUATION',
            'DATA_PRIVACY',
            'CONTAINER_SECURITY',

        );
    END IF;
END
$$;


/* ----- Wut wut --------------------------------------------- */
CREATE TABLE IF NOT EXISTS monitor_results (
    id UUID PRIMARY KEY,
    monitor_type "MonitorType" NOT NULL,
    monitor_result "MonitorResultType" NOT NULL,
    run_parameters JSONB NOT NULL,
);

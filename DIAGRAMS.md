# Mermaid Diagrams for Modules

## API Module (src/bin/api.rs)
```mermaid
flowchart TD
   A[Start: main()] --> B[Load Settings]
   B --> C[Connect to Database]
   C --> D[Create Repository]
   D --> E[Construct Env]
   E --> F[Build Router with routes: "/", "/id/:id", "/datasources", "/query"]
   F --> G[Serve Application]
```

## Planner Module (src/executor/planner.rs)
```mermaid
flowchart TD
   A[QueryPlanner::new(datasource)] --> B[plan(request)]
   B --> C[Extract Filters (start_date, end_date)]
   C --> D[Lookup Columns]
   D --> E[Generate Aggregation Query]
   E --> F[Join with Dim Data]
   F --> G[Return final Query (SqlAst)]
```

## Query Module (src/executor/query.rs)
```mermaid
flowchart TD
    A[SQLGenerator::new()]
    A --> B[generate_sql(ast)]
    B --> C[visit(ast)]
    C --> D{ast type}
    D -- Select --> E[Handle SELECT with: columns, FROM, WHERE, GROUP BY, ORDER BY]
    D -- Table --> F[Output Table (name & alias)]
    D -- Literal --> G[Output Literal value]
    D -- Logical --> H[Handle Logical (AND/OR)]
    D -- Comparison --> I[Output Comparison (columns & operator)]
```

## Settings Module (src/settings.rs)
```mermaid
flowchart TD
   A[Settings::new()] --> B[Load Config from resources/config]
   B --> C[Deserialize into Settings Struct]
```

## Domain Service Module (src/domain/service.rs)
```mermaid
flowchart TD
   A[ReportService::new(datasource)] --> B[create_report(request)]
   B --> C[Generate UUID for report id]
   C --> D[Set ReportStatus::Pending]
   D --> E[Return Report Object]
```

## API Handlers Module (src/api/handlers.rs)
```mermaid
flowchart TD
   A[root()] --> B[Return "Hello, World!"]
   A --> C[get_datasources()] --> D[Call repository.load_datasources()]
   A --> E[report()] --> F[Todo: Not implemented]
   A --> G[query()] --> H[Generate SQL query]
```

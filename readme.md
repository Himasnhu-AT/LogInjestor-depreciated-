# Log Injestor

> [!IMPORTANT]
> Depreciated
> Transferred to [Admin Nexus](https://github.com/Himasnhu-AT/adminNexus)

## Description

This is a simple log injestor that have three main components:

1. **Log Reader**: This is the main component that reads the log file and sends the logs to the log processor, which stores logs in db for later retrieval.

2. **Log Injestor**: HTTP server that receives logs from the log reader or api calls from applications and stores them in the database.

3. **Server**: This is the main server that runs the log injestor and log reader.

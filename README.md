### Conceptual plan...

- Go through list of folders in the settings file. For each folder...

- Find all the files ending in .log that were updated in the last 24 hours. For each file...

- Capture the last four error-entries and add them to a list.

---


### Fun to think about...

- What level of async to apply?

Thinking in queues here...

Max might be to...

- get-all-file-candidates:

    - get-file-candidates-per-folder

        - async for each folder, a glob-search for any .log files

        - for each file a glob-search returns, have a process that checks the date to add the file to a list of candidates.

- process-candidates:

    - for each candidate, have a process that reads through the file to look for and extract the last four error-entries.

---


### What do I want to return?


---


### Misc...

envy -- https://github.com/softprops/envy

---

# Team-json creator
 
This is very small, ugly and for almost nobody useful application, which parses data from CSV
and outputs JSON with people with their LinkedIn profiles URLs and photos.
 
It's here just to show how such utility (which would be normally implemented in Python or
such language) could be (quickly) implemented in Rust.
 
Input:
```
-- data
    | -- photos
    |       | -- name1.jpeg
    |       | -- name2.jpeg
    |
    |  -- data_1_group1.csv
    |  -- data_1_group2.csv
    |  -- data_1_group3.csv
```

Dirs are in format `data_(\d+)_(.*)\.csv` where first match group is group ID and second is the name
(both appear in resulting JSON, where groups are sorted by the ID).

CSV structure:
```csv
Name,LinkedIn
Name1 Surname1,https://www.linkedin.com/in/name1/
Name2 Surname2,https://www.linkedin.com/in/name2/
```

Name of the dir and prefix of the photo URLs are up to you.  
Run the program with args [dir with data] and [url prefix for photos]:

```bash
cargo run --release -- data https://your-prefix.for-url.photos/dir
```

You can uncomment the line starting with `webbrowser::open` to open your web browser with LinkedIn
profile for all records where the photo is not yet downloaded.  
If number of opened profiles is too high, you'll probably get banned ;-) (and also prepare some CPU power, RAM
and your OS may crash anyway, you never know)

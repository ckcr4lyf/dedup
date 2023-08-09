# dedup

Intented to be used to sort a bunch of images by date

Images have mixed dating information, some EXIF, some filename etc. The precedence order is based on the specific dataset. Feel free to fork and change as per your requirement.


## Running

Download `dedup.exe` , and then run it in a terminal using:

```
dedup.exe <path to folder to look into>
```

If the folder name has spaces, you can use quotes. Example:

```
dedup.exe "C:\Users\user\Pictures\Cool Folder Name"
```

## Logging

On Windows, set the environment variable in Powershell using:

```
$env:RUST_LOG = 'debug'
```
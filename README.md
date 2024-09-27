# Anonymize dicom files

# anonydcm

`anonydcm` is a command-line tool written in Rust that anonymizes DICOM files, allowing users to automatically anonymize and store DICOM files in different locations, or even in-place within the original folder.

## Features

- Anonymize DICOM files from a specified source directory.
- Automatically store anonymized files in the original folder or a specified destination.
- Rename anonymized files using custom names, including multi-part names.
- Recursive anonymization of DICOM files within subdirectories.
- Lightweight and efficient for batch anonymization tasks.

## Installation

To install the `anonydcm` tool, you'll need to have Rust installed on your system. You can install Rust [here](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/anonydcm.git
cd anonydcm
cargo build --release
```

This will create an executable in the `target/release/` directory.

## Usage

Here are some examples of how to use the `anonydcm` tool:

1. **Anonymize and store in a different location automatically:**

   ```bash
   anonydcm --src c:\dicom_folder
   ```

2. **Anonymize in the same folder:**

   ```bash
   anonydcm --src c:\dicom_folder -i
   ```

3. **Anonymize and store in a specific destination:**

   ```bash
   anonydcm --src c:\dicom_folder --dst c:\destination_anonymized_folder
   ```

4. **Anonymize and rename the files with a custom name (e.g., "sub1"):**

   ```bash
   anonydcm --src c:\dicom_folder --dst c:\destination_anonymized_folder --name sub1
   ```

5. **Anonymize and rename using a multi-part name (e.g., "John Brown"):**

   ```bash
   anonydcm -s c:\dicom_folder -d c:\destination_anonymized_folder -n "John Brown"
   ```

6. **Find and anonymize all DICOM files in a directory and its subdirectories:**

   ```bash
   anonydcm -s c:\dicom_folder -a
   ```

7. **Anonymize all DICOM files in subdirectories and rename them with a custom name:**

   ```bash
   anonydcm -s c:\dicom_folder -a -n sub1
   ```

### Options

- `--src` or `-s`: Specify the source folder containing DICOM files.
- `--dst` or `-d`: Specify the destination folder for anonymized files. If omitted, the files will be anonymized in a directory which will be created automatically[use of -i and -a blow, make the command to do anonimization in place].
- `--name` or `-n`: Specify a new name for the anonymized files.[default is empty but you can change it to whatever name you want]
- `-i`: In-place anonymization in the source folder.
- `-a`: Recursively find DICOM files in the specified directory and subdirectories.

## Contributing

Feel free to fork this repository and submit pull requests if you'd like to contribute to `anonydcm`. All contributions are welcome!

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.


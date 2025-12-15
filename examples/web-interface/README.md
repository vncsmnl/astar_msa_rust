# Web Interface for PA-Star

Flask-based web interface for the PA-Star Multiple Sequence Alignment tool.

## Prerequisites

- Python 3.7+
- Flask
- PA-Star binaries built in release mode

## Installation

```bash
cd examples/web-interface
pip install -r requirements.txt
```

## Running

```bash
python app.py
```

Then open your browser to `http://localhost:5000`

## Features

- Upload FASTA files
- Configure alignment parameters
- View alignment results
- Download aligned sequences

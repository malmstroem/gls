# gls

This repository contains the Rust-based data processing workflow developed for the research project currently under review.

## Note

This repository is associated with ongoing research, and the corresponding paper is not yet published.
While the core workflow is functional, additional refinements and updates may occur to align with the final publication.

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Building the Code](#building-the-code)
4. [Running the Code](#running-the-code)
5. [License](#license)

---

## Prerequisites

- **Tools/Software**: Mention the software/tools required, e.g.:
  - Rust (build and tested with version 1.83.0)
  - Cargo
  - Git
  - Docker

---

## Installation

- install openBIS (tested with the [openBIS 6.5 early access release](https://unlimited.ethz.ch/spaces/openbis/pages/240493370/Download+Early+Access+Release))
- install docker
- download resources from [zenodo:10.5281/zenodo.14292542](https://doi.org/10.5281/zenodo.14292542) - under embargo until the paper is published.
- import and start the wkde and gseapy containers

create the openBIS iwf rc file (~/.openbisrc.json)

```json
{
  "local": {
    "appurl": "https://localhost:8443/openbis/",
    "url": "https://localhost:8443/openbis/openbis/rmi-application-server-v3.json",
    "durl": "https://localhost:8444/datastore_server/rmi-data-store-server-v3.json",
    "v1url": "https://localhost:8443/openbis/openbis/rmi-query-v1.json",
    "uploadurl": "https://localhost:8444/datastore_server/session_workspace_file_upload",
    "username": "admin"
  }
}
```

---

## Building the Code

To build the project, use the following steps:

1. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

---

## Running the Code

1. After building, log into openbis
   ```bash
   ./target/release/gls local login
   ```

2. Setup the openBIS data model (requires openBIS admin rights)
   ```bash
   ./target/release/gls local setup
   ```

3. Create an iwf workflow object with the following parameters:


```json
{
  "name": "gls",
  "objects": {
    "input_0_1_1": {
      "target": "measure_v2.tsv",
      "compressed_target": "input_0_1_0.zip",
      "bucket": "bucket",
      "resource": "input_0_1_0.zip"
    }
  },
  "atlas_list": [
    "haatlas",
    "emblatlas",
    "mspatlas",
    "msratlas",
    "hacells",
    "emblcells"
  ],
  "protein_list": [],
  "protein_list_dev": [],
  "pl_4n": [],
  "nvar_grp": {},
  "f4g": [],
  "f3a": [],
  "f3b": [],
  "qmi": [
    {
      "filepath": "tissue_quantified_proteins.tsv",
      "name": "haatlasraw",
      "ex": true,
      "impute": false
    },
    {
      "filepath": "tissue_quantified_proteins.tsv",
      "name": "hacellsraw",
      "ex": true,
      "impute": false
    },
    {
      "filepath": "plasma_quantified_proteins.tsv",
      "name": "plnvar",
      "ex": true,
      "impute": false
    },
    {
      "filepath": "plasma_quantified_proteins.tsv",
      "name": "plmi",
      "ex": true,
      "impute": false
    },
    {
      "filepath": "plasma_quantified_proteins.tsv",
      "name": "plsepsis",
      "ex": true,
      "impute": false
    },
    {
      "filepath": "plasma_quantified_proteins.tsv",
      "name": "plpancr",
      "ex": true,
      "impute": false
    },
    {
      "filepath": "baseline_expression_counts-2020-05-07.tsv",
      "name": "emblatlas",
      "ex": false,
      "impute": false
    },
    {
      "filepath": "baseline_expression_counts-2020-05-07.tsv",
      "name": "emblcellsraw",
      "ex": false,
      "impute": false
    },
    {
      "filepath": "mmc3-e_protein_tissue_median.tsv",
      "name": "mspatlasraw",
      "ex": false,
      "impute": false
    },
    {
      "filepath": "mmc4-b_rna_tissue_median.tsv",
      "name": "msratlasraw",
      "ex": false,
      "impute": false
    }
  ],
  "gmt": "msigdb_v7.5.1_files_to_download_locally/msigdb_v7.5.1_GMTs/c5.go.bp.v7.5.1.symbols.gmt",
  "wkde_settings": {
    "fraction": 0.7,
    "bandwidth": 0.1,
    "min_value": 0.1,
    "no_class_criteria": 0.01,
    "max_n_labels": 2
  },
  "sample_map": {
    "nvar_01": [
      "p09",
      "t1"
    ],
    "nvar_02": [
      "p09",
      "t2"
    ],
    "nvar_03": [
      "p04",
      "t1"
    ],
    "nvar_04": [
      "p03",
      "t1"
    ],
    "nvar_05": [
      "p02",
      "t1"
    ],
    "nvar_06": [
      "p04",
      "t2"
    ],
    "nvar_07": [
      "p09",
      "t3"
    ],
    "nvar_08": [
      "p08",
      "t1"
    ],
    "nvar_09": [
      "p10",
      "t1"
    ],
    "nvar_10": [
      "p10",
      "t2"
    ],
    "nvar_11": [
      "p04",
      "t3"
    ],
    "nvar_12": [
      "p01",
      "t1"
    ],
    "nvar_13": [
      "p02",
      "t2"
    ],
    "nvar_14": [
      "p06",
      "t1"
    ],
    "nvar_15": [
      "p03",
      "t2"
    ],
    "nvar_16": [
      "p01",
      "t2"
    ],
    "nvar_17": [
      "p06",
      "t2"
    ],
    "nvar_18": [
      "p02",
      "t3"
    ],
    "nvar_19": [
      "p08",
      "t2"
    ],
    "nvar_20": [
      "p07",
      "t1"
    ],
    "nvar_21": [
      "p04",
      "t4"
    ],
    "nvar_22": [
      "p07",
      "t2"
    ],
    "nvar_23": [
      "p01",
      "t3"
    ],
    "nvar_24": [
      "p02",
      "t4"
    ],
    "nvar_25": [
      "p07",
      "t3"
    ],
    "nvar_26": [
      "p03",
      "t3"
    ],
    "nvar_27": [
      "p05",
      "t1"
    ],
    "nvar_28": [
      "p08",
      "t3"
    ],
    "nvar_29": [
      "p07",
      "t4"
    ],
    "nvar_30": [
      "p09",
      "t4"
    ],
    "nvar_31": [
      "p09",
      "t5"
    ],
    "nvar_32": [
      "p10",
      "t3"
    ],
    "nvar_33": [
      "p03",
      "t4"
    ],
    "nvar_34": [
      "p10",
      "t4"
    ],
    "nvar_35": [
      "p06",
      "t3"
    ],
    "nvar_36": [
      "p08",
      "t4"
    ],
    "nvar_37": [
      "p10",
      "t5"
    ],
    "nvar_38": [
      "p05",
      "t2"
    ],
    "nvar_39": [
      "p01",
      "t4"
    ],
    "nvar_40": [
      "p08",
      "t5"
    ],
    "nvar_41": [
      "p06",
      "t4"
    ],
    "nvar_42": [
      "p07",
      "t5"
    ],
    "nvar_43": [
      "p05",
      "t3"
    ],
    "nvar_44": [
      "p02",
      "t5"
    ],
    "nvar_45": [
      "p06",
      "t5"
    ],
    "nvar_46": [
      "p04",
      "t5"
    ],
    "nvar_47": [
      "p05",
      "t4"
    ],
    "nvar_48": [
      "p01",
      "t5"
    ],
    "nvar_49": [
      "p03",
      "t5"
    ],
    "nvar_50": [
      "p05",
      "t5"
    ]
  }
}
```

4. Create an iwf settings object with the following parameters, make sure to change the db and os credentials and the wf env paths:

```json
{
  "wf_env": {
    "base_path": "path",
    "binds": [
      "path"
    ]
  },
  "execs": {
    "default": {
      "Native": {}
    },
    "wkde": {
      "DockerExec": {
        "name": "wkde"
      }
    },
    "gseapy": {
      "DockerExec": {
        "name": "gseapy"
      }
    }
  },
  "submit_settings": null,
  "dbcredentials": {
    "Postgres": {
      "user": "user",
      "database": "db",
      "password": "pwd",
      "root_user": "root_user",
      "root_password": "root_password",
      "host": "localhost"
    }
  },
  "oscredentials": {
    "endpoint": "endpoint",
    "key": "key",
    "secret": "secret"
  }
}
```

5. Run the workflow
   ```bash
   ./target/release/gls local run -w WORKFLOW_OBJECT -s SETTINGS_OBJECT
   ```

---

## License

This project is licensed under the [MIT License](LICENSE).

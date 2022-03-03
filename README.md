# Store and retrieve data from the NFDI4BioDiv Core-Storage

This small example shows how to upload and retrieve data from the [NFDI4BioDiv Core-Storage](https://kb.gfbio.org/display/NFDI/CORE-Storage).
Please first get familiar with the general architecture described [here](https://github.com/ScienceObjectsDB/Documentation).

## Preliminaries
In this guide we use the development instance of the storage. Currently (due to the lack of an AAI), all data is 
organized in *projects* that are manually created on the following website:

https://website.scienceobjectsdb.nfdi-dev.gi.denbi.de

After logging in with your github account, please create a new project and a corresponding API token. This token is 
required to access the CORE-Storage APIs.

The API Endpoint is located at https://api.scienceobjectsdb.nfdi-dev.gi.denbi.de/swagger-ui/

## Overview
The steps required to access the storage are as follows:

- Log in to the website as described above.
- Create a project and a corresponding API Token.
- Use the API to create data sets and upload your data.
- Download your data.

## Data Organization
The CORE-Storage organizes data in hierarchies. At the top-level there is a **DataSet**. A dataset contains one or more 
**ObjectGroups**. Those groups are meant to group data (e.g., files) that belong closely together, like a data and
and index file. Every **ObjectGroup** contains one or more objects. Objects are the basic unit of storage and in most 
cases correspond to a file.

In this example, we create a simple **DataSet** with one **ObjectGroup** that consists of a single ***Object**.

## API Usage
In the source folder you find an example application written in the [rust](https://www.rust-lang.org/) programming 
language. It uses the stubs provided [here](https://github.com/ScienceObjectsDB/rust-api). However, there are 
implementations for other programming languages available (a full list is available [here](https://github.com/ScienceObjectsDB/Documentation#implementations).

The examaple creates a single **DataSet** with one **ObjectGroup** that consists of a single ***Object**. In order to 
run it, you must insert a valid *project id* and a valid *API token*.


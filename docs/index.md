---
layout: page
homepage: true
---

# Overview

Trappo is a remote server automation and deployment tool written in Rust.
It's easy to install and use so that you can focus on writing your application instead.

# Installation

If you don't have cargo already installed on your machine,
[install](https://github.com/rust-lang/cargo/) it first.

Then run the following command to install trappo:

```bash
$ cargo install trappo
```

That's it! trappo is now installed on your system.

# Basic Usage

## Main Configuration file

Configuration files are located in a folder called `.trappo` in the root of the project you want to deploy using trappo.
The main configuration file read by trappo is `.trappo/config.toml`.
This file defines basic configuration such as where different servers are located and the repository git url.

```toml
[global]
host            = "my-server"
repo-url        = "git@github.com:dcabrejas/example-project.git"
linked-files    = ['app/etc/env.php', 'public/gtm.txt']
linked-dirs     = ['public/media', 'public/feeds']

[develop]
deploy-path     = "/var/www/my-website/___deploy"
keep-releases   = 3
branch          = "develop"

[staging]
deploy-path     = "/my-website-staging/___deploy"
keep-releases   = 1
branch          = "master"
```

In this example we have defined two stages : Staging and Develop.
Both will deploy our repo `git@github.com:dcabrejas/example-project.git` to the same server `my-server`
and both will use symlinks to symlink files and folders that persist across deployments.

However each stage will be deployed to a different directory, the git branch and the maximum number of releases we
want to keep for each one is also different.

**Global Section**

This section overrides stage specific configuration. It's a good place to put configuration that
is shared by all your different stages such as `repo_url`.

**Host**

This value is a valid host where you can ssh into, this is usually an alias you have configured in your `~/.ssh/config`
file but could also be anything that would normally follow an `ssh` command, such as `www-data@my-server.com -p 5555`.

## Defining custom steps

Custom steps commands can be on the server by defining them in .trappo/steps.toml.
You can define when in the deployment process each command is run, they are run from the release directory.

```toml
[global]
  [[global.steps]]
  name    = "custom:composer:install"
  command = "composer install"
  after   = "git:clone"
[develop]
  [[staging.steps]]
  name    = "notify:webhook"
  command = "curl https://www.third-party/endpoint"
  after   = "core:cleanup:releases"
```

As with the general configuration, the general section can be used to define steps that need to run on all stages.
In this example we have defined 2 custom steps to be run as part of our deployment process.

**custom:composer:install**

This step will be run after the default trappo step `git:clone` which clones the git repository on the server.
More on default steps later.
Composer install is used in PHP applications to install dependencies.

**notify:webhook**

This step will be run only when we deploy to the staging server.
It makes a request to a third party URL after the deployment has finished.

## Deploying

Deploying is as simple as running the following command from the root of your project.

```bash
$ trappo deploy <stage>
```

So for example deploying to staging would be :

```bash
$ trappo deploy staging
```

## Rolling back

Rolling back is as simple as running the following command from the root of your project.

```bash
$ trappo rollback <stage>
```

So for example rolling back the latest release to develop would be :

```bash
$ trappo rollback develop
```

## Help

See basic usage help by running the following command:

```bash
$ trappo --help
```

# Server Layout

## Main Structure

When you deploy your project for the first time using trappo, the following layout of directories will be created on the server
at the deployment path specified in the configuration.

```bash
.
├── current -> releases/20180924143501
├── releases
│  ├── 20180922191500
│  └── 20180924143501
└── shared
```

- **current:**  Symlink pointing to the current release.
- **releases:** Directory where releases are stored, each release is deployed to a folder named using the date time at which it got deployed.
- **shared:** Directory where you put files that are persisted across releases. You use file and directory symlinks to use these from the release directory.

## Symlinks

Symlinks can be configured in the `.trappo/config.toml` file and they will be created automatically.

For example given the following configuration example :

```toml
[develop]
linked-files    = ['.env']
linked-dirs     = ['media']
```

This is what it would look like on the server:

```bash
.
├── current -> releases/20180924143501
├── releases
│  ├── 20180922191500
│  └── 20180924143501
│     ├── .env -> ../../shared/.env
│     ├── index.php
│     └── media -> ../../shared/media
└── shared
   ├── .env
   └── media
```

This way you can keep images and environment related files on the server and they will be automatically used for every release
without them needing to be part of your repo.

# Stages Configuration

Stage configuration is placed in the following file `.trappo/config.toml`.

Configuration defined in the global namespace applies to every stage and overrides configuration defined for other stages if they have the same name.

Here is a list of a stage configuration using all possible options:

```toml
[staging]
host            = "my-server"
repo-url        = "git@github.com:dcabrejas/example-project.git"
linked-files    = ['app/etc/env.php', 'public/gtm.txt']
linked-dirs     = ['public/media', 'public/feeds']
deploy-path     = "/var/www/my-website/___deploy"
keep-releases   = 3
branch          = "develop"
```

**host**

This value is a valid host where you can ssh into, this is usually an alias you have configured in your `~/.ssh/config`
file but could also be anything that would normally follow an `ssh` command, such as `www-data@my-server.com -p 5555`.

**repo-url**

Your repo's git url. trappo will clone the repository on the server using this url.

**linked-files** and **linked-dirs**

Files and directories which need to be symlinked from the `shared` directory.

**deploy-path**

Path on the server where the project will be deployed, it's an absolute path.

**keep-releases**

Maximum number of releases to keep on the server. After each deploy, old releases will be cleaned up.
Set a number here that makes sense for your needs, in production you probably want to have more than one release to allow you to rollback,
whereas on staging you might want to keep only one to save disk space.

**branch**

What git branch of your project do you want to run on this environment.

# Define custom steps.

Custom steps can be defined to run at arbitrary stages of the development process in the following file: `.trappo/steps`

Steps defined in the global namespace are run for every stage you define and override steps defined for other stages if they have the same name.

When defining custom steps, you have to specify whether they run after or before another step, here is a list of core steps that are always run by trappo, which you can reference in your custom steps:

**core:init:** Creates the directory layout on the server

**git:clone:** Clone the git repo on the server

**core:link:files:** Symlinks files from the shared directory

**core:link:directories:** Symlinks directories from the shared directory

**core:link:current:** Creates the current symlink pointing to the latest release

**core:cleanup:releases:** Cleans up old releases

Example usage :

```toml
[global]
  [[global.steps]]
  name    = "custom:composer:install"
  command = "composer install"
  after   = "git:clone"
[develop]
  [[develop.steps]]
  name    = "notify:webhook"
  command = "curl https://www.third-party/endpoint"
  after   = "core:cleanup:releases"
```

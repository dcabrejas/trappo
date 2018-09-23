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

```
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

```
$ trappo deploy <stage>
```

So for example deploying to staging would be :

```
$ trappo deploy staging
```

## Rolling back

Rolling back is as simple as running the following command from the root of your project.

```
$ trappo rollback <stage>
```

So for example rolling back the latest release to develop would be :

```
$ trappo rollback develop
```

## Help

See basic usage help by running the following command:

```
$ trappo --help
```

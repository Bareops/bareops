# DSL Specification


## How to define comments

TODO: use //

## How to define task

* A task starts with a keyword "task" follow by the task name
* A task is scoped by curly braces
* A task contains a plugin (aka action) definition
* A task has an optional output
* A task can be assigned to 0-n tags

## How to define multiline strings?

TODO:

Ideas:
* use double quotes for every line to clearly define where whitespaces starts and ends
* use special characters to encapsulate the whole string to define if lines are separated by newline or not
* use backslash to quote double quotes?
* best option might be to define a template or text file instead of adding multiline strings

# How to handle different package managers

* try to match system type to pkgmanager, like ubuntu = apt, fedora = dnf, suse = zypper, arch = pacman, windows = ?
* for alternatives add overwrite options

## How to transfer task output between tasks

TODO: describe, see example

## How to do conditional tasks

TODO: describe, see example

## How to do loop over items in tasks

TODO: describe, see example

## How to use file templates processed by tasks

Maybe use Askama as template engine?

## What happens if outputs of tasks match outputs of other tasks

* shadow until scope is left, but what is the scope?
* override?

## How are lists of tasks called inside of one file?

The is called `taskbook`, since it only contains a linear list of tasks.

## How does someone specify the target nodes for taskbooks

CLI call ideas:

* `bareops run <taskbook>.bl -t <targets> --tags all_tasks,webservers`

Targets are a list of ips or hostnames which can be reached by SSH.
Tags restrict the execution of certain tasks. Tags restrict execution to certain targets.

TODO: Target file structure



## How to we define variables that will be inserted/used during runtime?

TODO: structure needed


## Syntax examples, work in progress

```

// General syntax:

task "remote plugin" {
  tags: ["mytag","mytag2"] 

  remotepluginname {
    url: http://topplugins.com/plugin1.wasm
    option1: value1
    option2: value2 
  }
  > remoteoutput 
}

task "taskname" {
  tags: ["mytag","mytag2"] 

  pluginname {
    option1: value1
    option2: value2 
  }
  > foo 
}

include "includename" {
  file "full or relative path"
}

task "taskname" {
  pluginname {
    option1: foo // the foo value from first task 
    option2: loopitem
  }
  < loopitem in foo
  ? foo >= 10 && foo <= 100 &&
    foo >= 10 && foo <= 100 ||
    (foo >= 10 && foo <= 100)
  > bar; // Output is None if conditional resolves to false
}

task "taskname" {
  pluginname {
    option1: bar // the bar value from second plugin 
    option2: +"Lorem ipsum dolor sit amet, consetetur sadipscing elitr,"
        "sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat,"
        "sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. "
        "Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet." 
        "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod "
        "tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. "
        "At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren," 
        "no sea takimata sanctus est Lorem ipsum dolor sit amet."+
    option3: abc
  } 
  > bar;
} 

#######

task "Install nginx {
  package {
    name: "nginx"
    status: installed | removed
    version: "123"
    update_cache: yes
    options: {
      manager: "npm" // overwrite package manager
    }
  }
}

task "Add user to group root" {
  account {
    name: "{{ user }}"
    groups: root
    status: active | inactive | removed
  }
}

task "Remove default nginx configuration" {
  file {
    path: /etc/nginx/sites-enabled/default
    status: exists | removed
  }
}

task "Add nginx configuration" {
  copy {
    source_folder: <control-node-path-to>/static-site-config
    target_folder: /etc/nginx/sites-available/
    folder: /etc/nginx/sites-available/
    owner: "{{ user }}"
    group: root
    mode: '0644'
  }
}

task "Enable website configuration" {
  file {
    source: /etc/nginx/sites-available/static-site-config
    target: /etc/nginx/sites-enabled/static-site-config
    status: link | removed
  }
}

task "Ensure nginx is running" {
  service {
    name: nginx
    state: started
    enabled: yes
  }
}


task "Install Package" {
    apt_install "nginx";
}

task "Start Service" {
    systemd_start "nginx";
}

```

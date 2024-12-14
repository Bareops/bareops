# DSL Specification


## How to define comments

use //

## How to define task

* A task starts with a keyword "task" follow by the task name
* A task is scoped by curly braches
* task parameters are simple key value pairs separated by colon
* task parameters are seperated by newlines

## How to define multiline strings?

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


## How to use templates
Maybe use askama as template engine?

## Syntax examples, work in progress

```

// General syntax:
task "taskname" {
  tags: ["mytag","mytag2"] 

  pluginname {
    option1: value1
    option2: value2 
  } 
  => foo;

  pluginname {
    option1: foo // the foo value from first plugin 
    option2: loopitem
  }
  < loopitem in foo
  ? foo >= 10 && foo <= 100 &&
    foo >= 10 && foo <= 100 ||
    (foo >= 10 && foo <= 100)
  > bar; // Output is None if conditional resolves to false
  
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
  => bar;
  
  (foo, bar) // the return value of the tasks
} 
=> (foo, bar)

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
  }

  permissions {
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

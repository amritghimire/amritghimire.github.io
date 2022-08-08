

As my first opensource and first rust project, I created an opensource command line interface for day to day activity of JIRA.  
  
  
I have a habit of automating most of my day to day task to terminal command. I have created a command line application that can be used to interact with Jira from terminal.
 


The package can be found in crates.io as [jira-terminal](https://crates.io/crates/jira-terminal) and aur repository as [jira-terminal-bin](https://aur.archlinux.org/packages/jira-terminal-bin/)
 


The link to github is **<https://github.com/amritghimire/jira-terminal>**


 
The following features are implemented in the package:

* List and filter the issues in JIRA with Jira filters, jql, or other fields.
* Move a ticket between the list of transitions available for a ticket.
* Add and use most used jql, component, user and other values as alias.
* View the details of a ticket along with comments from a ticket.
* View the list of a fields available for a ticket including custom fields.
* Update the fields for a ticket.
* Create a new issue or sub tasks.
* List or add new comments to the issue.



   

## Installation


This application can be used in multiple platform.


### Arch Linux


This package is availabe in aur repository as [jira-terminal-bin](https://aur.archlinux.org/packages/jira-terminal-bin/)


### Debian/Ubuntu


On debian based system, the deb file is available in [releases](https://github.com/amritghimire/jira-terminal/releases). You can download latest release from there. Please make sure libc is installed in your system.


### Cargo


If you already have a Rust environment set up, you can use the cargo install command: `cargo install jira-terminal`


Cargo will build the jira-terminal binary and place it in $HOME/.cargo/bin. You can also setup Rust toolchain from [Rust official site](https://www.rust-lang.org/tools/install)


### Manual Installation from Github


Compiled binary versions of jira-terminal are uploaded to GitHub when a release is made. You can install jira-terminal manually by [downloading a release](https://github.com/amritghimire/jira-terminal/releases) , extracting it, and copying the binary to a directory in your $PATH, such as /usr/local/bin.


## Usage


When running the application for first time, you will be asked with following values.


* hostname [This will be used to identify the jira hostname to be used.]
* email [Email address you use to login with the application.]
* token [You can obtain the app password from the link specified in the application]


After that, you can use following commands for help.




```
jira-terminal help
jira-terminal help list
jira-terminal help transition
jira-terminal help alias
jira-terminal help detail
jira-terminal help fields
jira-terminal help update
jira-terminal help new
jira-terminal help assign
jira-terminal help comment

```





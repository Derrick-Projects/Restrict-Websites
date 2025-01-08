## INTRODUCTION
&nbsp;&nbsp;&nbsp;&nbsp; A Command-Line Rust Application for restricting access to inserted websites. The general idea is to aid command-line programmers, who may be easily distracted from their tasks.
A user simply inputs the websites they don't want to access and my application does that rest. This will help users stay focused on their task at hand. It may also help other users who maybe addicted to social media scrolling, or general internet addiction.

&nbsp;&nbsp;&nbsp;&nbsp; Additionally, there are two distinct features incorporated into the program; the first one is a confirmation prompt before restricting a website, and the second is using regular expression to make sure the website entered, either begins with "https://" or "http://". 
If the website inputted doesn't start with any of those two expressions, the program will send out an error message and a prompt to enter the website URL correctly.

## INSTALLATION
&nbsp;&nbsp; Download the Repository:
- Go to this URL: [My Repository](https://github.com/Derrick-Projects/Restrict-Websites)
- Click on the green "Code" button.
- Select "Download ZIP" to download the repository as a ZIP file.
- Extract the zip to a folder of your choosing in local machine.
- Open your terminal or git bash.
- Cd to the downloaded zip folder located in the folder you extracted to.
- Type the command cargo run to build and run the rust project.

&nbsp;&nbsp; Clone the Repository:
- Open your terminal or git bash.
- Change the directory to where you want to clone the repository.
- Clone the repository using the following command: git clone https://github.com/Derrick-Projects/Restrict-Websites.git
- Change into the cloned repository directory.
- Type the command cargo run to build and run the rust project.

## Basic Commands:
* Add a Website to block/restrict a specific website: cargo run
* List/Display Blocked Websites: cargo run -- --list

![Screenshot of Restrict-Website](Images/Screenshot%20(123).png)



![Screenshot of Restrict-Website](Images/Screenshot%20(124).png)


## USAGE
**Productivity Enhancement**: Individuals can block distracting websites during work hours to improve their focus and concentration.

**Self-Discipline**: Users can set personal goals/boundaries for reducing time spent on social media, gaming websites, or general internet addiction by blocking them.

## Problems Solved
**Distraction Management**: The application aids users in managing distractions by limiting access to non-essential websites.

**Content Control**: Allows users to prevent content they don't want to access especially in a work setting.

**Network Security**: Reduces the risk of malware or phishing attacks by blocking access to harmful sites.

**Increased Workplace Productivity**: Assists employers in maintaining a productive work environment by controlling access to distracting sites or by employees limiting themselves and staying focused.

**Time Management**: Improves better time management by allowing users to set limits on certain website usage.

## Advantages
**User-Friendly**: The command-line interface application is straightforward for users familiar with terminal commands.

**Performance**: Rust is known for its performance and memory safety, making the application efficient and reliable to execute.

**Cross-Platformed**: Can run on various operating systems (Windows, macOS, Linux), making it versatile for different users.

**Lightweight**: The application operates on minimal resource consumption compared to heavier GUI applications.

**Privacy**: The application operates locally without sending data to external servers, ensuring user privacy


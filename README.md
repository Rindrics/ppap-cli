# PPAP CLI 🔐

> "*Here comes the ZIP, doo-doo-doo-doo, Here comes the pass, and I say: It's secure.*"
>
> — George Has-risk-son, '*Here comes the ZIP*'

PPAP (**P**assword-locked zipfile **P**assword ***A**ngouka* **P**rotocol) is a traditional Japanese file sharing protocol.
PPAP CLI streamlines the PPAP-based file sharing process by enabling zip compression and subsequent password sharing by single command.

## What is PPAP?

PPAP is a widely adopted file sharing protocol in Japanese business culture, known for its unique two-step approach to secure file sharing:

1. Send a password-protected zip file
2. Send the password in a separate email

Based on PPAP, which consists of two steps, files containing sensitive information can be shared securely (debatable).

## Prerequisites

- SendGrid API key
- Verified "mail-from" email address

----

:construction: WIP

## Installation

```bash
cargo install ppap
```

## Configuration

Create a `.env` file with the following settings:

```env
SENDGRID_API_KEY=your_api_key_here
SENDGRID_PROTOCOL=rest  # or smtp
EMAIL_FROM_ADDRESS=your.verified@example.com
```

## Usage

```
ppap [OPTIONS] <FILE> <EMAIL>

ARGUMENTS:
    <FILE>        File to be encrypted and sent
    <EMAIL>       Recipient's email address

OPTIONS:
    -s, --secure            Enhance security by sending an incorrect password
                           (Good luck explaining this to your recipient)
    -a, --after <HOURS>     Delay password delivery by specified hours
                           (Because waiting makes everything more secure)
    -h, --help             Print help information
    -V, --version          Print version information

EXAMPLES:
    # Simple usage - sends file and password immediately
    ppap document.pdf recipient@example.com

    # Maximum security mode - no one can open the file
    ppap --secure document.pdf recipient@example.com

    # For the recipient with patience - sends password 3 hours later
    ppap --after 3 document.pdf recipient@example.com

    # For the extra careful - recipients will weep for security that exceeds their expectations
    ppap --secure --after 2 document.pdf recipient@example.com

NOTES:
    - The --secure option follows the time-honored tradition of "user verification through confusion"
    - The --after option helps ensure your recipient is really committed to receiving that file
    - Remember: The more steps in your security protocol, the more secure it becomes! 
      (At least that's what we tell ourselves)
```

### Example

```bash
ppap document.pdf recipient@example.com
```

This will:
1. Create a password-protected zip file
2. Send the zip file to the recipient
3. Send the password in a separate email

## Contributing

Contributions to improve the security of this tool are welcome!
Please feel free to submit a Pull Request.
For important changes, please open an issue first and discuss what you want to change.

## License

This project is licensed under the MIT License.

## Quotes

<details>
  <summary>Give me one if you have anything suggestive</summary>
 
> "*While others pursue quantum encryption, we perfect the art of security through strategic inconvenience.*"
>
> — Dr. 404 NotFound, Journal of Impractical Computing, 2023

> "*When your file sharing protocol requires a flowchart to explain, you know you're doing security right.*"
>
> — Prof. Chaos Theory, Department of Unnecessary Complexity

> "*The best security measure is the one that makes users question their career choices.*"
>
> — Ms. Git Merge-Conflict, Tales from Tech Support

> "*Remember: if your users aren't sighing heavily while following the protocol, it's not secure enough.*"
>
> — Dr. UX Nightmare, Principles of Digital Masochism

> "*The strength of a security protocol is directly proportional to the number of emails required to complete a single file transfer.*"
>
> — Lord of the Pings, Network Mythology

> "*The complexity of human behavior remains our most reliable encryption algorithm.*"
>
> — Dr. Grace Cooper, Behavioral Cryptography Journal

> "*True security lies not in the strength of our algorithms, but in the patience of our users.*"
>
> — Alan Turin, Institute of Computational Psychology

> "*Security is like a pen-pineapple-apple-pen: it doesn't have to make sense to be effective.*"
> 
> — Pico Taro

> "*Sometimes the most effective firewall is a series of bureaucratic procedures.*"
>
> — Sarah HashCode, Enterprise Security Patterns

> "*In the digital age, the human element remains our most unpredictable—and therefore valuable—security component.*"
>
> — Dr. Binary Tree, Human-Computer Interaction Studies

> "Who dares call my methods vulnerable?!"
>
> — Dr. Max Strongcipher

> "*Everything should be made as simple as possible, but not simpler than PPAP.*"
>
> — Albert Enumstein, On the General Theory of File Sharing

> "*Patient you must be. Send file first you will. Password later comes. Much later. Hmmmm...*"
>
> — Master Coda, The Last Security

> "*My mama always said file sharing was like a game of ping pong. Back and forth, back and forth.*"
>
> — Forest Dump, Memoirs of a System Administrator

> "*Security is not about building walls; it's about creating rituals.*"
>
> — Zen Master IPv6, Digital Tea Ceremony

</details>

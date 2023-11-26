# Secret Santa CLI

- Get a list of friends and their email addresses
- Pair them up for secret santa
- Let them know!

## Usage

You will need a `participants.csv` like the following:

```
name,email
Alice,alice@example.com
Bob,bob@example.com
```

Create a `.aws_credentials` file with:

```bash
$ cp .aws_credentials.example .aws_credentials
```

Get your AWS SES secrets and add them to the new file, then simply run:

```bash
$ cargo run -- 
```

To generate couples and send the emails.
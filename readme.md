# Focused Resume Creator (FRC)
![GitHub Issues](https://img.shields.io/github/issues/jbilinski/profile-manager-frc)
![License: MIT](https://img.shields.io/badge/License-MIT-success.svg?logo)
![Package Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fgithub.com%2Fjbilinski%2Fprofile-manager-frc%2Fraw%2Frefs%2Fheads%2Fmain%2Ffrc-service%2FCargo.toml&query=%24.package.version&label=version)


The goal of this projcet is to build a job-targeted resume based on a source resume of all skills and experiance.  If the target job is not a good fit the resume will be incomplete and output will advise against submission 

### Inputs
- A JSON resume file
- Personal physosophy and approach to work
- A job description or URL to a job description
- Company values or URL to company values
- Switch: 1:1 Qualification match
- Switch: Termiology matching
- Switch: First Acronym Expansion
- Switch: SFIA matching

### Returns
 - QUALS Advisor
 - Modification Factor



### Environment Variables
use a `.env` file to set the following environment variables:

```
OPEN_API_KEY=open_api_key
BIND_IP=website_bind_ip_address (default 127.0.0.1)
BIND_PORT=website_bind_port_number (default 9086)
#DEFAULT_RESUME_JSON=path_to_default_resume_json
#TEMPLATE_PATH=your_template_path
```

### GitHub actions
- Update and build json resume struct
- Build docker container


### Source and Research Credits
- [JSON Resume](https://jsonresume.org) for the resume schema
- [SFIA](https://sfia-online.org/en/about-sfia) Skills Framework for the Information Age standard reference model
- [references](docs/references.bib) formal citations
- [Indeed posting template](https://www.indeed.com/hire/c/info/writing-a-job-posting-template-and-examples)
- [Reading A Federal Job Posting (US GSA TTS)](https://tts.gsa.gov/join/federal-job-posting/)
- [OPM (US) Job posting requirements](https://www.opm.gov/policy-data-oversight/human-capital-management/hiring-reform/hiring-process-analysis-tool/create-and-post-a-job-opportunity-announcement-including-identifying-career-patterns/)

### Other resume prompt writing tips
- [US GSA UX expert guidance](https://handbook.tts.gsa.gov/hiring-staying-or-changing-jobs/resume/)
- [USJOBS What to include](https://help.usajobs.gov/faq/application/documents/resume/what-to-include)
- [GS Pay Tables](https://www.opm.gov/policy-data-oversight/pay-leave/salaries-wages/) Assists in reverse match of employment history to GS levels

### General Technical References
- [Rust OpenAI API library](https://github.com/64bit/async-openai)
- [OpenAI](https://openai.com)
- [GitHub Actions](https://docs.github.com/en/actions)
- [Docker](https://docs.docker.com)
- [Docker Compose](https://docs.docker.com/compose)


### License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details


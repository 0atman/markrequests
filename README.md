## Development and discussion on the [Project](https://github.com/users/0atman/projects/3)

# Markrequests

A plain-text Issue and PR tracking system.

An example of a file to track these is in [prs/example.md](https://github.com/0atman/markrequests/blob/main/prs/example.md)



## Rationale

I am unsatisfied with the conclusions I reached in my "Plain Text Teams" video. (https://www.youtube.com/watch?v=WgV6M1LyfNY) It's difficult to separate my desire for purity in doing as much in plain-text as possible, with the practicality that GitHub (and other web hosts) have some features you're not going to want to miss, but can't yet get in plain text.  
   
So, in the video, I proposed a compromise. Yes, Issues, Projects and even PRs are outside of git, but because they are tied so closely to your repo, I still recommended them, as a concession to practicality.  
   
I still prefer GitHub projects to JIRA because it's the same tool, bound up closely to the repository, where the work is actually happening. My assumption is that you'll never change your git host, so these add-on tools (that are broadly similar in all GitHub competitors) are ALSO never going away. The Ulysses Pact remains!  
   
I'm considering building a git-native PRs, issues and projects system, backed into markdown files inside the repo in prs/, issues/, projects/, if they're stored in markdown, you can build them in whatever editors you would like, they only need a CLI tool and web viewer to present them, everything else can be done git and plain text native? I'll make a full GH project soon and announce it when I've made a simple reference implementation CLI (simpl yaml and markdown parsing are gonna make this an easy initial MVP project) Here's a mockup of a PR template to show you what I mean https://gist.github.com/0atman/11c80138148fcba126a2c88b1893ff34

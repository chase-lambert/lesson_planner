# lesson_planner

Full stack Rust app that helps teachers build lesson plans. It is currently deployed at https://lessonplanner.onrender.com/ during the early dev stage. Please note that it is using render's free tier so the first page load may be slow while it wakes up. 

I have been a teacher for many years and one of the hardest, most time consuming processes us teachers encounter is lesson planning and creating the corresponding materials. I wanted a project that would allow me to explore new technologies while also solving a real world problem that I encounter on a daily basis.

The backend is built with Axum and will use a Postgres db (probably with supabase). I plan on using OpenAI's chatGPT as I've found it creates surprisingly high quality results for this domain but until that API is ready I am using their davinci model. Payment processing will be done through Stripe. Deployment is through render. 

The frontend is currently using Askama templates with htmx for interactivity and Tailwind CSS for styling. I also plan on exploring Leptos to gain experience with the cutting edge in Rust frontend development.

This will be a full stack app backing a real business with user authentication and payment processing. This will remain public while I am in the current early stages building out the foundation for others to use (and allowing me to use it for portfolio purposes in my current job search) but I may take it private with no notice in the near future.

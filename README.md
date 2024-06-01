# pea
pea (plan and execute agents) is a framework for building planning agents in rust


pea
- planner agent which will create steps and final answer
- executor agent which will call tools to perform actions
- pea will be the equivalent to the AgentExecutor in that it will iterate on the steps called by the planner as well as updating the intermediate steps (scratchpad) to pass to the planner
- also will manage memory and execute tasks
- the task will be passed to it which will have a set number of tools or use the default tools passed to pea
- the execute task method will take in the task, and run the planner + executor flow with the tools
- there needs to be an output parser that will parse the though + action response from the planner agent (which will be a ReAct agent)
- the planner agent takes in the goal, backstory, and role
- the executor really is performing the expected action, with a tool and returning a completed step (action, observation)
- the (action, observation) is consumed into intermediate steps
- so the pea runs with a while loop that checks should_continue and then iterates on each step. each step calls the planner agent and gets a list of actions or a final answer. if it's a list of actions the actions are looped through and the executor will call the tools and return the output (action, observation). The main while loop then starts again with the (action, observation) pairs added to intermediate steps
- I'm not sure how a manager/supervisor best plays into this, but will figure that out when expanding to multi agents

```rust

use std::collections::HashMap;

trait pea {
    fn planner(prompt: String, goal: String, backstory: String, role: String, intermediate_steps: Vec<String>) {}
    fn executor(prompt: String, tools: Vec<Tool>) {}
    fn execute_task(task: Task) {}
    fn tools(tools: Vec<Tool>) {}
    fn max_iterations() {}
    fn should_contine(max_iterations: u32) -> Bool {}
    fn run(&self, inputs: HashMap<str, str>) -> String {
        while should_continue() {
            for actions_or_answer in self.plan(self.intermediate_steps, inputs).iter() {
                match actions_or_answer {
                    Actions(actions) => {
                        for action in actions {
                            let observation = self.execute(action);
                            let mut map = HashMap::new();
                            map.insert(action, observation);
                            self.intermediate_steps.push(map);
                        }
                    }
                    Answer(answer) => {
                        return answer;
                    }
                }
            }
        }
    }
}
```


The planner will need to take in a prompt and prompt params. Will need to run as a ReAct agent and have parsable outputs

First Goal: Build planner


The executor will need to take in the action and the available tools. It will need to return an observation. Along with the action taken

Second Goal: Build the executor


Pea will need to consume the result into intermediate steps to pass back to the planner

Third Goal: Build run loop for pea
Fourth Goal: Pass tools, prompt params into pea


Specify a task and a desired output to make sure the task is completed.

Fifth Goal: Use task description and desired output to create a prompt that can be passed to pea


Then multiple agents and managers can be added to the mix.

Sixth Goal: Add pod executor
Seventh Goal: Add manager/supervisor support
Eighth Goal: Add agent delegation support
Ninth Goal: Support multi-agents

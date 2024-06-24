use git2::{Commit, Repository};
fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(err) => panic!("failed to open repository: {}", err),
    };

    // Get the HEAD reference
    let head = match repo.head() {
        Ok(head) => head,
        Err(err) => panic!("failed to get HEAD: {}", err),
    };

    // Get the commit object pointed to by HEAD
    let commit_id = head.target().unwrap();
    let commit = match repo.find_commit(commit_id) {
        Ok(commit) => commit,
        Err(err) => panic!("failed to find commit: {}", err),
    };

    // Access information about the commit (optional)
    println!("HEAD Commit SHA:\n{}", commit.id().to_string());
    println!(
        "HEAD Commit message:\n{}",
        commit.message().unwrap_or("No message")
    );
    // Create a vector to store commit history
    let mut commit_history: Vec<String> = Vec::new();

    // Get the first parent (if it exists)
    let index = 0;
    let parent_commit = match commit.parent(index) {
        Ok(parent) => parent,
        Err(err) => {
            println!("This commit has no parent (likely the initial commit).");
            return;
        }
        Err(err) => panic!("failed to get parent commit: {}", err),
    };
    println!("Commit SHA:\n{}", parent_commit.id().to_string());
    println!(
        "Parent Commit message:\n{}",
        parent_commit.message().unwrap_or("No message")
    );

    //// Iterate through the commit history starting from HEAD
    //let mut current_commit = Some(commit);
    //while let Some(ref commit) = current_commit {
    //    let commit_message = commit.message().unwrap_or("No message");
    //    println!("Commit message: {}", commit.message().unwrap_or("No message"));
    //    //commit_history.push(&commit_message);
    //    commit_history.push((&commit_message).to_string());

    //    current_commit = commit.parents().next().map(|p| p.peel::<Commit>(commit).unwrap());
    //}

    //// Create a vector to store XOR of each commit
    //let mut xor_history: Vec<String> = Vec::new();
    //for i in 0..commit_history.len() - 1 {
    //    //let xor_result = vector_commit::xor_strings(&commit_history[i], &commit_history[i + 1]);
    //    //xor_history.push(xor_result);
    //}

    //// Print the commit history
    //println!("Commit History:");
    //for commit in commit_history {
    //    println!("{}", commit);
    //}

    //// Print the XOR history
    //println!("XOR History:");
    //for xor in xor_history {
    //    println!("{}", xor);
    //}
}

// fn main() {
//     let repo_path = "/path/to/your/git/repository"; // Replace with your repo path
//
//     // Open the git repository
//     let repo = match Repository::open(repo_path) {
//         Ok(repo) => repo,
//         Err(err) => panic!("Failed to open repository: {}", err),
//     };
//
//     // Get the head commit
//     let head_commit = match repo.head() {
//         Ok(Some(head)) => head.peel::<Commit>(Commit),
//         Ok(None) => panic!("No HEAD found in the repository"),
//         Err(err) => panic!("Failed to get HEAD commit: {}", err),
//     };
//
//     // Create a vector to store commit history
//     let mut commit_history: Vec<String> = Vec::new();
//
//     // Iterate through the commit history starting from HEAD
//     let mut current_commit = Some(head_commit);
//     while let Some(commit) = current_commit {
//         let commit_message = commit.summary().unwrap().to_string();
//         commit_history.push(commit_message);
//
//         current_commit = commit.parents().next().map(|p| p.peel::<Commit>(Commit).unwrap());
//     }
//
//     // Create a vector to store XOR of each commit
//     let mut xor_history: Vec<String> = Vec::new();
//     for i in 0..commit_history.len() - 1 {
//         let xor_result = vector_commit::xor_strings(&commit_history[i], &commit_history[i + 1]);
//         xor_history.push(xor_result);
//     }
//
//     // Print the commit history
//     println!("Commit History:");
//     for commit in commit_history {
//         println!("{}", commit);
//     }
//
//     // Print the XOR history
//     println!("XOR History:");
//     for xor in xor_history {
//         println!("{}", xor);
//     }
// }

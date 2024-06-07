words = "Hey I am Intern at QuadB Tech Company "

word = words.split()
shortest = word[0]
for i in word[1:]: 
    if len(i) < len(shortest):  
        shortest = i

print(shortest)

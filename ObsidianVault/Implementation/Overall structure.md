### Game data structure 
```mermaid
flowchart TD
G[[Grid]]
C[[Cells]]
H[[Snake heads]]
S[[Snake bodies]]


G -- coordinates --> C
C --> H 
C --> S 
H -. move forward/kill event .-> S 
G -. game loop .-> H
```

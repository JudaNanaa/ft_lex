%%
[0-9]+  { return 1; }
[a-z]+  { return 2; }
.       { return 3; }
%%

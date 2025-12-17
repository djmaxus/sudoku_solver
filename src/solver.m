board = [
    0,0,3,4,5,6,9,0,8;
    0,0,7,1,3,9,5,0,2;
    0,5,9,0,7,0,0,0,0;
    2,0,6,0,4,5,7,0,0;
    0,0,0,8,1,0,4,9,6;
    9,8,4,3,0,7,0,2,0;
    5,0,8,6,0,0,2,1,0;
    0,4,1,0,0,3,0,5,9;
    0,9,2,0,8,0,0,3,4;
    ];

row_sums = zeros(9,numel(board)+1);
col_sums = zeros(9,numel(board)+1);
block_sums = zeros(9,numel(board)+1);

for i = 1:9
    row_sum_sub = zeros(size(board));
    row_sum_sub(i,:) = 1;
    row_sums(i,:) = [row_sum_sub(:)',sum(1:9)];
end

for i = 1:9
    col_sum_sub = zeros(size(board));
    col_sum_sub(:,i) = 1;
    col_sums(i,:) = [col_sum_sub(:)',sum(1:9)];
end

for i = 1:3
    for j = 1:3
       block_sum_sub = zeros(size(board));
       block_sum_sub((3*(i-1)+1):(3*i),(3*(j-1)+1):(3*j)) = 1;
       block_sums((i-1)*3+j,:) = [block_sum_sub(:)', sum(1:9)];
    end
end

constraints = [
    row_sums;
    col_sums;
    block_sums];

% known values
[I,J,V] = find(board);
known_vals = zeros(numel(V),numel(board)+1);

for idx=1:numel(V)
    i = I(idx);
    j = J(idx);
    val_constr = zeros(size(board));
    val_constr(i,j) = 1;
    known_vals(idx,1:end-1) = val_constr(:);
    known_vals(idx,end) = V(idx);
end

% build system
sudoku_system = [constraints;known_vals];
A = sudoku_system(:,1:end-1);
b = sudoku_system(:,end);
x = ones(numel(board),1)*5;
r = b - A*x;
while norm(r) > eps
    dx = A \ r;
    x = x + dx;
    x = max(x,1);
    x = min(x,9);
    r = b - A*x;
    disp(norm(r));
end


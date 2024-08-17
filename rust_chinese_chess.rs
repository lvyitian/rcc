use rand::Rng;
use std::sync::RwLock;
use rand::thread_rng; // 0.8.4
#[macro_use] extern crate lazy_static; // 1.4.0
use std::sync::atomic::AtomicU8;
use std::io::stdin;
fn get_rand()->rand::rngs::ThreadRng
{
    return thread_rng();
}
const TMP_NONE: Option<Box<Action>>=None;
lazy_static!{
  static ref ACTIONS: RwLock<[Option<Box<Action>>; 64]>=RwLock::new([TMP_NONE;64]);
  static ref ACTIONS_NUM: AtomicU8=AtomicU8::new(0);
}
#[derive(Clone)]
struct Coord{
    x:u8,
    y:u8
}
impl Coord{
    pub fn new(x:u8,y:u8)->Coord
    {
        return Coord{x:x,y:y};
    }
}
#[derive(Clone)]
struct Action{
    src:Coord,
    tar:Coord,
    removed:i8
}
impl Action
{
    pub fn new(x1:u8,y1:u8,x2:u8,y2:u8)->Action
    {
        return Action{src:Coord::new(x1,y1),tar:Coord::new(x2,y2),removed:0};
    }
    pub fn from(&mut self,value:&Action)
    {
        let tmp=&value.src;
        self.src=tmp.clone();
        let tmp=&value.tar;
        self.tar=tmp.clone();
        self.removed=value.removed;
    }
}
fn copy_board(tar:&mut [[i8;9];10],src:[[i8;9];10])
{
    for i in 0..9
    {
        for j in 0..10
        {
            tar[j][i]=src[j][i];
        }
    }
}
#[allow(dead_code)]
fn undo(board:&mut [[i8;9];10])
{
    ACTIONS_NUM.fetch_sub(1,std::sync::atomic::Ordering::SeqCst);
    {
        let tmp_act=ACTIONS.read().unwrap();
        let t_act=&*tmp_act;
        let tmp2=t_act[ACTIONS_NUM.load(std::sync::atomic::Ordering::SeqCst) as usize].as_ref();
        let tmp3=tmp2.unwrap();
        board[tmp3.src.y as usize][tmp3.src.x as usize]=board[tmp3.tar.y as usize][tmp3.tar.x as usize];
	    board[tmp3.tar.y as usize][tmp3.tar.x as usize]=tmp3.removed;
    }
}
fn do_action(board:&mut [[i8;9];10],act:&mut Box<Action>,record:bool)
{
    act.removed=board[act.tar.y as usize][act.tar.x as usize];
    board[act.tar.y as usize][act.tar.x as usize]=board[act.src.y as usize][act.src.x as usize];
	board[act.src.y as usize][act.src.x as usize]=0;
	if record
	{
	  {
        let mut tmp_act=ACTIONS.write().unwrap();
        let t_act=&mut *tmp_act;
        t_act[ACTIONS_NUM.load(std::sync::atomic::Ordering::SeqCst) as usize]=Some(Box::new(*act.clone()));
      }
    ACTIONS_NUM.fetch_add(1,std::sync::atomic::Ordering::SeqCst);
	}
}
fn get_score(board:[[i8;9];10])->i16
{
    let mut r:i16=0;
    for i in 0..9
    {
        for j in 0..10
        {
            r+=board[j][i] as i16;
        }
    }
    return r;
}
fn abs(value:i8)->i8
{
    if value<0
    {
        return -value;
    }
    return value;
}
fn search_next(board:[[i8;9];10],is_ai:bool,act:&mut Box<Action>,depth:u8,alpha:i16,beta:i16)->i16
{
    let mut temp_board=[[0;9];10];
    copy_board(&mut temp_board,board);
    do_action(&mut temp_board,act,false);
    if abs(act.removed)==127
    {
        return (((depth+1)%2*2-1) as i16*get_score(temp_board)) as i16;
    }
    return -search(&mut None,temp_board,!is_ai,depth-1,-beta,-alpha) as i16;
}
fn is_in_board(x:u8,y:u8)->bool
{
    return x<9&&y<10;
}
fn is_space(board:[[i8;9];10],x:u8,y:u8)->bool
{
    if !is_in_board(x,y)
    {
        return false;
    }
    return board[y as usize][x as usize]==0;
}
fn can_move(board:[[i8;9];10],is_ai:bool,x:u8,y:u8)->bool
{
    if !is_in_board(x,y)
    {
        return false;
    }
    return board[y as usize][x as usize]==0||(board[y as usize][x as usize]>0)!=is_ai;
}
fn is_in_nine_palaces(is_ai:bool,x:u8,y:u8)->bool
{
    return x>2&&x<6&&(if is_ai {y>6&&y<10} else {y<3});
}
macro_rules! next_step {
    ($actptr:ident,$board:ident,$is_ai:ident,$act:ident,$depth:ident,$alpha:ident,$beta:ident)=>{
        let val:i16=search_next($board,$is_ai,$act.as_mut().unwrap(),$depth,$alpha,$beta);
	                  if val>=$beta
	                  {
	                      if let Some(actptrv)=&mut *$actptr
	                      {
	                          actptrv.from($act.as_ref().unwrap());
	                      }
	                      return $beta;
	                  }
	                  if val>$alpha
	                  {
	                   if let Some(actptrv)=&mut *$actptr
	                   {
	                      actptrv.from($act.as_ref().unwrap());
	                   }
	                   $alpha=val;  
	                  }
    }
}
fn search(actptr:&mut Option<Box<Action>>,board:[[i8;9];10],is_ai:bool,depth:u8,mut alpha:i16,beta:i16)->i16
{
    if depth==0
    {
        return get_score(board);
    }
    let r:bool=get_rand().gen();
    let mut i:i8=if r {0} else {8};
    while if r {i<9} else {i>=0}
    {
        let r1:bool=get_rand().gen();
        let mut j:i8=if r1 {0} else {9};
        while if r1 {j<10} else {j>=0}
        {
          if board[j as usize][i as usize]!=0&&(board[j as usize][i as usize]>0)==is_ai
	      { 
	      let mut act:Option<Box<Action>>;
	      match abs(board[j as usize][i as usize])
	      {
	          1=>{
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              act.as_mut().unwrap().tar.y=(act.as_ref().unwrap().tar.y as i8+(if is_ai {-1} else {1}) as i8) as u8;
	              if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	              }
	              if if is_ai {j<5} else {j>4}
	              {
	                  act.as_mut().unwrap().tar.y=j as u8;
	                  act.as_mut().unwrap().tar.x-=1;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  act.as_mut().unwrap().tar.x+=2;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	              }
	          },
	          2=>{
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              act.as_mut().unwrap().tar.x+=1;
	              act.as_mut().unwrap().tar.y+=1;
	              if is_in_nine_palaces(is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)&&can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	              }
	              act.as_mut().unwrap().tar.x-=2;
	              if is_in_nine_palaces(is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)&&can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	              }
	              act.as_mut().unwrap().tar.y-=2;
	              if is_in_nine_palaces(is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)&&can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	              }
	              act.as_mut().unwrap().tar.x+=2;
	              if is_in_nine_palaces(is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)&&can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	              }
	          },
	          3=>{
	              if is_ai||j<4
	              {
	                  act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	                  act.as_mut().unwrap().tar.x+=1;
	                  act.as_mut().unwrap().tar.y+=1;
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                      act.as_mut().unwrap().tar.x+=1;
	                      act.as_mut().unwrap().tar.y+=1;
	                      if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                      {
	                        next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                      }
	                  }
	                  act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	                  act.as_mut().unwrap().tar.x-=1;
	                  act.as_mut().unwrap().tar.y+=1;
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                      act.as_mut().unwrap().tar.x-=1;
	                      act.as_mut().unwrap().tar.y+=1;
	                      if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                      {
	                        next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                      }
	                  }
	              }
	              if j>5||!is_ai
	              {
	                  act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	                  act.as_mut().unwrap().tar.x+=1;
	                  act.as_mut().unwrap().tar.y-=1;
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                      act.as_mut().unwrap().tar.x+=1;
	                      act.as_mut().unwrap().tar.y-=1;
	                      if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                      {
	                        next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                      }
	                  }
	                  act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	                  act.as_mut().unwrap().tar.x-=1;
	                  act.as_mut().unwrap().tar.y-=1;
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                      act.as_mut().unwrap().tar.x-=1;
	                      act.as_mut().unwrap().tar.y-=1;
	                      if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                      {
	                        next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                      }
	                  }
	              }
	          },
	          5=>{
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              act.as_mut().unwrap().tar.x+=1;
	              if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  act.as_mut().unwrap().tar.x+=1;
	                  act.as_mut().unwrap().tar.y+=1;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                       next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  act.as_mut().unwrap().tar.y-=2;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                       next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              act.as_mut().unwrap().tar.x-=1;
	              if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  act.as_mut().unwrap().tar.x-=1;
	                  act.as_mut().unwrap().tar.y+=1;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                       next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  act.as_mut().unwrap().tar.y-=2;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                       next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              act.as_mut().unwrap().tar.y-=1;
	              if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  act.as_mut().unwrap().tar.y-=1;
	                  act.as_mut().unwrap().tar.x+=1;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                       next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  act.as_mut().unwrap().tar.x-=2;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                       next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              act.as_mut().unwrap().tar.y+=1;
	              if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  act.as_mut().unwrap().tar.y+=1;
	                  act.as_mut().unwrap().tar.x+=1;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                       next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  act.as_mut().unwrap().tar.x-=2;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                       next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	              }
	          },
	          6=>{
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              loop{
	                  act.as_mut().unwrap().tar.x+=1;
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                      next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }else{
	                      if is_in_board(act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                      {
	                          loop{
	                              act.as_mut().unwrap().tar.x+=1;
	                              if is_in_board(act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                              {
	                                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                                  {
	                                      continue;
	                                  }
	                                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                                  {
	                                    next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                                  }
	                              }
	                              break;
	                          }
	                      }
	                      break;
	                  }
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              loop{
	                  act.as_mut().unwrap().tar.x-=1;
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                      next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }else{
	                      if is_in_board(act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                      {
	                          loop{
	                              act.as_mut().unwrap().tar.x-=1;
	                              if is_in_board(act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                              {
	                                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                                  {
	                                      continue;
	                                  }
	                                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                                  {
	                                    next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                                  }
	                              }
	                              break;
	                          }
	                      }
	                      break;
	                  }
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              loop{
	                  act.as_mut().unwrap().tar.y+=1;
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                      next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }else{
	                      if is_in_board(act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                      {
	                          loop{
	                              act.as_mut().unwrap().tar.y+=1;
	                              if is_in_board(act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                              {
	                                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                                  {
	                                      continue;
	                                  }
	                                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                                  {
	                                    next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                                  }
	                              }
	                              break;
	                          }
	                      }
	                      break;
	                  }
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              loop{
	                  act.as_mut().unwrap().tar.y-=1;
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                      next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }else{
	                      if is_in_board(act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                      {
	                          loop{
	                              act.as_mut().unwrap().tar.y-=1;
	                              if is_in_board(act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                              {
	                                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                                  {
	                                      continue;
	                                  }
	                                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                                  {
	                                    next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                                  }
	                              }
	                              break;
	                          }
	                      }
	                      break;
	                  }
	              }
	          },
	          10=>{
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              loop{
	                  act.as_mut().unwrap().tar.x+=1;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                    next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                    continue;
	                  }  
	                  break;
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              loop{
	                  act.as_mut().unwrap().tar.x-=1;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                    next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                    continue;
	                  }  
	                  break;
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              loop{
	                  act.as_mut().unwrap().tar.y+=1;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                    next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                    continue;
	                  }  
	                  break;
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              loop{
	                  act.as_mut().unwrap().tar.y-=1;
	                  if can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                    next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                    continue;
	                  }  
	                  break;
	              }
	          },
	          127=>{
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              act.as_mut().unwrap().tar.x+=1;
	              if is_in_nine_palaces(is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)&&can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	              }
	              act.as_mut().unwrap().tar.x-=2;
	              if is_in_nine_palaces(is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)&&can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              act.as_mut().unwrap().tar.y+=1;
	              if is_in_nine_palaces(is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)&&can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	              }
	              act.as_mut().unwrap().tar.y-=2;
	              if is_in_nine_palaces(is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)&&can_move(board,is_ai,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	              {
	                  next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	              }
	              act=Some(Box::new(Action::new(i as u8,j as u8,i as u8,j as u8)));
	              loop{
	                  act.as_mut().unwrap().tar.y=(act.as_ref().unwrap().tar.y as i8 + (if is_ai {-1} else {1}) as i8)as u8;
	                  if is_in_board(act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                  {
	                      if is_space(board,act.as_ref().unwrap().tar.x,act.as_ref().unwrap().tar.y)
	                      {
	                       continue;
	                      }  
	                  if abs(board[act.as_ref().unwrap().tar.y as usize][act.as_ref().unwrap().tar.x as usize])==127
	                  {
	                      next_step!(actptr,board,is_ai,act,depth,alpha,beta);
	                  }
	                  }
	                  break;
	              }
	          },
	          _=>panic!()
	      }
	      }
	      if r1
	      {
	          j+=1;
	      }else{
	          j-=1;
	      }
        }
        if r
        {
            i+=1;
        }else{
            i-=1;
        }
    }
    return alpha;
}
fn run_ai(board:&mut [[i8;9];10])->Box<Action>
{
    let mut act=Option::Some(Box::new(Action::new(0,0,0,0)));
    println!("score: {}",search(&mut act,*board,true,4,-2000,2000));
    do_action(board,act.as_mut().unwrap(),true);
    return act.unwrap();
}
fn main() {
    let mut board:[[i8;9];10]=[[-10,-5,-3,-2,-127,-2,-3,-5,-10],[0,0,0,0,0,0,0,0,0],[0,-6,0,0,0,0,0,-6,0],[-1,0,-1,0,-1,0,-1,0,-1],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[1,0,1,0,1,0,1,0,1],[0,6,0,0,0,0,0,6,0],[0,0,0,0,0,0,0,0,0],[10,5,3,2,127,2,3,5,10]];
    //println!("{:?}",board);
    
    /*let mut tmp:[[i8;9];10]=[[0;9];10];
    println!("{:?}",tmp);
    copy_board(&mut tmp,board);
    println!("{:?}",tmp);*/
    let scanf=stdin();
    loop
    {
        let mut act=run_ai(&mut board);
        println!("{},{} {},{} : {} -> {}",act.src.x,act.src.y,act.tar.x,act.tar.y,board[act.tar.y as usize][act.tar.x as usize],act.removed);
        let mut buf=String::new();
        scanf.read_line(&mut buf).unwrap();
	buf=buf.trim().to_string();
        let x1=buf.parse::<u8>().unwrap();
        buf=String::new();
        scanf.read_line(&mut buf).unwrap();
	buf=buf.trim().to_string();
        let y1=buf.parse::<u8>().unwrap();
        buf=String::new();
        scanf.read_line(&mut buf).unwrap();
	buf=buf.trim().to_string();
        let x2=buf.parse::<u8>().unwrap();
        buf=String::new();
        scanf.read_line(&mut buf).unwrap();
        buf=buf.trim().to_string();
	let y2=buf.parse::<u8>().unwrap();
        //buf=String::new();
        act=Box::new(Action::new(x1,y1,x2,y2)); 
        do_action(&mut board,&mut act,true);
    }
}

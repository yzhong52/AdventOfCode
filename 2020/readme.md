# AdventOfCode

Advent of Code 2020 in Python.

![](advant_of_code_2020.png)

## Day 11: Seating System

```
#~#~ #~#~# ~#~#~#~#~#~#~#~# ~ #~#~#~ #~#~  # ~#~#~##~ #~#~#~ #~#~  #~#~#~#~#~#~#~#~#~#~#~#~#~#~#~#
~~~~~~~~~~~~~~~~~~~~~~~~~~~ #~~ ~~~~~~~~~ ~  #~~~~~~# ~~~~~# ~~~~#~~~~~~~~~~~~~~~~~ ~~~~~~~~~~~~~~
#~#~#~#~#~ #~#~#~#~#~# ~#~# ~~~#~#~# #~#~##~#~~#~##~~~~#~#~~ #~#~~ ~# ~#~~ #~ #~#~#~~#~#~#~#~#~#~#
~~~ ~~~~~~~~~~~ ~~~~ #~#~~~~#~# ~~ # ~~~~~~~ ~~~~~~~# ~~~~~# ~~~~# ~~#~~~# ~~~~~~~~ ~~~~~ ~~~~~~~~
#~#~~#~#~# ~#~#~#~#~~~~~~#~ ~~~~ ~~~ ~#~# ~#~#~#~##~~ ~#~#~~~#~#~~ #~~~#~~ ##~#~#~~ #~#~#~~#~#~#~#
~~~~#~~~~# ~~~~~~~~~ #~#~#~#~#~#~### ~~~~~~~~~~~~~~~~#~~~~~# ~~~~#~~~# ~~#~~~~~~~~# ~~#~~~~~~~~~~~
   ~  #~ ~~ #~#    ~      ~  #     ~     #    ~ ##     ~#  ~ #   ~~ ~   ~~  # ~   ~ #~   #  ~#  ~ 
#~#~~#~~#~#~~~~~#~ ~ ~#~#~# ~~~~#~~~ ~#~#~ # ~#~~~~~#~#~~~## ~#~#~ #~~#~#~#~~ ~#~## ~#~#~~~# ~~#~#
~~~~#~~~~ ~~#~#~~~#~ ~~~~~~~#~#~#~#~ ~~~~~#~ #~~#~#~~~#~#~~~ ~~~~~ #~~~~~~ #~#~~~~~ ~~~#~#~~~#~~~ 
#~#~~~#~#~~~~~~~#~~~#~#~#~#~~~~~~~~~ #~#~~~~#~~~~~~~# ~~~~#~#~#~#~ ~~#~#~# ~~~~#~#~ #~~~~~~#~~~#~~
~~~~#~~~~~#~#~#~~~#~~~~~~~# ~ #~#~ # ~~~~# ~ ~#~#~#~~ #~#~~~~~~~~~ #~~~~~~ #~#~~~~~ #~#~# ~~#~~~~#
#~#~~~#  ~ ~~~~~#~~  #~# ~~ ~~~~~~ ~ #~#~~~# ~~~~~~~# ~~~#~# ~##~# ~~ #~## ~~~~#~#~ ~~~~ ~#~~~#~~~
   ~  ~ ~ ~      ~    ~ ~  #   ~ ~  # ~   #~ #~    ~   #        ~ ~ ~~~    #  ~  ~   ~ #  ~  ~~   
~#~#~#~#~# ~~#~#~#~~#~#~#~~ ~ #~#~#~ ~~~#~~~ ~~#~~#~~#~~#~#~ ~~~~~ #~#~~~~~~~#~ # ~#~#~~~#~#~#~#~#
#~~~~~~~~~~#~~~~~~~~ ~~~~ # ~~~~~~~~~#~#~~#  ~~~~~~~~ ~~~~~~ #~#~# ~~~~#~#~#~~~~~#~ ~~~#~~~~~~~~~~
~~# ~#~ #~ ~~#~#~#~# ~#~#~  ##~#~#~# ~~~~~~~~##~##~#  ~#~#~#~~~~~~ #~#~~~~ ~~#~#~~# ~~~ ~#~#~#~#~#
 ~#~#~~~~~ #~ ~~~~~~ #~~ ~~~~~~~~~~~~#~#~#~# ~~~~~~~~~~~~~~~ #~#~# ~~~~#~~ #~~~~~~~ ~# ~~~~~~~~~~~
#~~~~ #~#~#~~~#~#~## ~~#~#~ #~#~#~#~ ~~~~~~~~#~#~#~#~#~#~#~# ~~~~~ #~#~~~# ~#~#~#~#~~~~#~#~#~#~#~#
 ~~#~#~~~~~~~~~~~~#~ # ~~~~ ~~~~~~~~ #~#~#~#~#~~~#~~~~~~  ~~ #~#~#~~~~~#~~ ~~~~~~~~#~#~~~~~~~~~~~~
~    #~     #      ~~~      # ~#~  #    ~ ~    #    ~# ~ ~# ~~        ~~    # ~ #  ~~~~       ~   
#~#~#~~ #~#~~~~#~#~#~##~#~#~~~#~~#~~~~#~#~#~#~ ~~~#~~~#~#~~# ~#~#~ ~~# ~#~ ~~~#~~~# ~#~#~#~#~#~#~#
 ~~  ~~ ~~ ~~#~~~~~~ #~ ~~~~#~~~~~~# ~~~~~~~ ~#~#~ ~#~~~~~~~ ~~~~~## ~#~~~#~#~~~#~~~~~~~~~~~~~~~~~
~~# ~#~#~#~#~~~~#~#~ ~~#~#~#~~#~#~~~~#~#~#~# ~~~~~ ~~ ~#~#~~ ~#~#~ ~#~~~#~ ~ ~# ~#~#~#~#~#~#~#~#~#
#~#~~~~~~~  ~~#~~~~~ #~~~ ~ ~#~ ~~#~~~~~~~~~ ##~#~~## ~~~~~# ~~~~  ~~~#~~#~#~  ~~~~ ~~~~~~~~~~~~~~
~~~ #~#~#~ ~#~~~#~#~ ~~#~~# ~~~#~#~  #~#~#~#  ~~~~~~~ ~#~#~~ #~#~# ##~~~~~ ~~~~~#~#~~#~#~#~#~#~#~#
 ~#~~~~~~~#~~~#~~~~~ #~~~~~ #~ ~~~~# ~~~~~~~~~~#~#~~# ~~~~~#~~~~~~~~~~#~#~#~#~#~~~~#~~~~~~~~~~~~~~
#~~~#~#~#~~~#~~~~#~# ~~ ~## ~~#~#~~~ ~#~#~#~ #~~~~~~~  ~#~ ~ #~#~# ~#~~~~~ ~~~~~#   ~#~#~#~#~#~#~#
~~# ~~~~~~ #~~~#~~~~~#~#~~~ #~~~~~## ~~~~~~~~~~#~#~~#~#~~~#~ ~~~~~ ~~#~#~# ~#~#~~~# ~~~~~~~~~ ~~~~
#~~~ ~#~#~   #~#~#~#~#~~ #~ ~~#~#~~~ #~##~#~ #~~~~~~~~~~~~~  #~#~~ #~#~~~~  #~#~#~# ~#~#~#~#~~~#~#
  ~   ~~~        ~~    ~     ~     #  ~  ~     ~ ~ #~   #      ~~#~~ ~ ~           ~ ~~     ~#   ~
~#~#~#~#~#~~~~~~#~#~ ~#~#~~ #~~#~#~~~~~~~~~~ ~#~#~##~ #~~~#~#~~#~~ ~#~#~#~ #~#~#~#~ #~#~#~#~~~#~~#
#~~~~~~~~~ # ~#~ ~~~ #~~~~# ~~#~~~~# ~#~#~ ##~~~~~~~~ #~#~~~~~~~~# ~~~~~~~~#~#~#~#~ #~~~~~~ #~~~~~
~~#~#~#~#~#~~~~~~#~#~~~#~~~~#~~~#~~~ #~~~~~~ ~#~#~##~ ~~~~#~ #~#~~ #~#~#~# ~~~~~~~~ ~~#~#~#~~~ #~#
#~~~~~~~~~~~#~#~ ~~~~#~~~#~ ~~#~~~#~~~~#~ ## ~~ ~~~~~##~#~~~ #~#~# ~~~~~~~~~#~# ~# ~#~ ~~~~~~#~~~~
~~#~#~#~~# ~~~~~#~#~#~~#~~~ #~~~#~~   ~~~~~~~#~#~#~#~ ~~~~#~ ~~~~~ #~#~#~# ~~~~~~~# ~#~#~#~#~~# ~#
#~#~~~~ ~# ~#~#~~~~~ ~#~~#~#~~~~#~#~ ##~#~#~  ~~~~~~~ #~#~~~ ~#~## ~~~~~~~ ~#~#~#~~ ~~~~~~~~~~~~~~
    ~  # ~ ~~  ~#~  ~    ~~   # ~  ~~ ~      ~   #~# ~ ~ #     ~       #      ~~     #  ~ #  ~#   
~#~#~#~~~~ ~#~~~~~#~ #~#~#~ #~~~##~# ~~#~#~ ~#~#~~~~~#~#~~~#~#~~~~ ~~#~~#~ #~#~#~~ #~~#~#~#~#~~~ #
#~~~~~~#~# ~~~#~#~~~ #~~~ ~ #~# ~~~~ #~#~#~# ~~~~#~#~~~~#~~~ #~#~# ~~~~~~~~~~~~~~#~ ~~~~~~~~~#~#~~
~~#~#~#~~~~#~ ~~~~#~ ~~#~~#~~~~~#~#   ~~~~~~ #~#~~~~~~#~~~#~~~~~~~ #~#~#~#~#~#~#~~~ ~#~#~#~#~~~~~#
#~~~~~~~#~ ~~##~#~~~~#~ ~~~~#~#~~  ~#~~#~#~#~~~~~##~# ~~#~~~# ~#~# ~~~~~~~ ~~~~~#~# ~~~ ~~~~~#~#~~
~~#~#~#~~~ # ~~~~~~# ~~~~#~~~~~~##~#~~~~~~~~ #~#~ ~~~ #~#~#~ ~~~~~~~#~#~#~#~#~#~~~~ #~# ~#~#~~~~~#
#~ ~~~~~#~ ~~#~#~#~~ #~#~#~ #~#~#~~~ #~#~#~#~ ~~~~~#~ ~~~ #~ #~#~# ~~~~~~~ ~~~~~~~# ~~~~~~~~~#~#~~
  ~# ~#~   #~~     ~ ~ ~      ~      ~        ~ ##      ~~~  #~  ~  ~ ~#~  ~#  ~# ~     ~ #~    ~ 
~#~~~#~~~~ #~ #~#~#~#~#~#~#~~#~#~#~#~~#~#~#~#~~#~~#~~ #~#~#~~~#~##~#~#~~~# ~~~#~~~#  ~#~# ~~#~#~~#
#~~#~~~~#~ ~~~#~#~#~~~#~#~# ~#~#~#~~ #~~#~#~##~#~~ ~# ~~~~#~ ~ ~~~~~~#~# ~ ~#~~~ ~~ # ~~~#~~~~~#~~
~~#~~~#~~~ ~#~~~~~~~ ~~~~~~~~ ~~~#~# ~~~~~~~~~~~~##~~ #~#~~~ #~#~# ~~~~~~~ ~~~~~#~# ~#~#~#~#~#~~~#
#~#~#~~~~ #~~~~~~~#~#~~~#~# ~~#~~~~~~~~#~#~~ #~#~~~~#~~~~~ ~##~#~~ #~~##~# ~~ #~#~~ ~~~~~~~~~~~ ~~
~~~~~~###~~~#~#~#~#~ ~#~#~#~# ~~#~#~##~~~~~#~#~#~ #~~~#~#~#~ ~~~~~ ~~# ~~~ ~#~#~ ~# #~##~#~#~#~#~#
~  ~  ~~  #~~   ~      ~     ~   ~~  ~  #     ~ ~   #~      ~ # ~#~ ~  #    ~    ~   ~ ~        ~~
#~#~~~#~#~ #~~~#~#~#~~~#~#~~~~~ ~#~~ ##~#~#~#~#~##~~~~~#~~~#~~~~~~~#~~~~~~ #~#~#~#  ~~~~#~~~~~~~~#
~~#~#~ ~~# ~~#~~~~~~ #~~~~~#~#~#~~~# ~ ~~ #~#~#~~~~#~#~~~#~~ #~#~#~~~#~#~~~#~#~~~~~ #~#~~ ~#~#~#~~
# ~~~~~#~~ #~~~#~#~#~~~#~#~ ~~~~~#~~ #~#~~~~~~~#~~#~~ ~#~~ # ~~~~~ #~~~~~#~~~~~#~## ~~~~##~#~~~~~#
~~~#~#~~~# ~~# ~~ ~~ #~~~~~~#~ #~ ~~ ~~~~#~#~#~~~~~~# ~~~#~~~~#~##~~~#~#~~ ##~#~ ~~ #~#~~~~~~#~#~~
#~ ~~~~#~~~#~~~#~ #~ ~#~#~#~~~~~~~~  #~#~~~~~~~ ### ~  #~~~# ~~~~~  #~~~~# ~~~~~#~~~~~~~#~~#~~~~~#
~~##~#~~~#~~~# ~#~~~#~~~~~~ ~#~#~#~# ~~~ #~##~#~~~~~# ~~~#~~ ~#~## ~~~#~~~  #~#~~~# ~#~~~~#~~#~#~~
#~~~~~~#~~ #~~ ~~~#~ #~ ~#~#~~~~~~~~ #~#~~~~ ~~~#~#~~ ~#~~~# ~~~~~ ~#~~~#~ ~~~ ~# ~ ~#~#~~~ ~~~ ~ 
~#~#~#~~~#~~#~#~#~#~ ~~#~ ~~~#~#~#~# ~~~ #~# ~#~~~~~# ~~~#~~ ~#~#~#~~~#~~~#~#~ ~ ~# ~~~~~#~#~# ~~#
  ~    #    ~    ~        #   ~    ~  #~ ~ #  ~ ###~ ~ #~~   ~   ~  #~  #   ~    ~    #       ~ ~ 
#~~~~~~~#~  ~~~ ~#~#~~#~#~# ~~#~#~#~ ~~~ ~~~  #~~~~~~ ~~~~#~ #~#~# ~~~~~~~~#~#~#~#  ~~  ~~~~~~~#~~
~~#~#~#~~~ #~#~#~~~~ ~~~~~~~#~#~#~#~ #~~~#~~ ~~ #~#~# ~~#~#~ #~~~~~~#~#~#  ~~#~#~~~~~#~#~#~#~#~~~#
#~~~~~~~# ~~~~~~~#~#~#~#~##  ~~~~~~# ~~#~~~#~#~~~~~~~ #~~~~~ #~#~# ~~~~~~~~~#~~~~~# ~~~~ ~~~~~~#~~
~~#~#~#~~~ #~#~#~~~# ~~~~~~  #~#~#~~  ~~~#~# ~~ ##~##~~~#~#~ ~~~~~ ~#~#~## ~~~#~#~~~#~#~#~#~#~~~ #
# ~~~~~~~# ~~~~~~#~~ ~~ #~~ ~~~~~~~#~#~#~~~  ~#~~~~~~~#~~~~~ #~#~# ~~~~~~~ #~~~~~~# ~~~~~~~~~~##~~
~~##~#~#~~  #~# ~~~##~#~#~#~#~#~#~~~ ~~~~#~ #~~~#~~#~ ~~#~#  #~~~~ ~#~#~#~ ~~#~#~#~~#~#~#~#~#~~~~#
#     ~  #~~     #~ ~  ~  ~~    ~~#~~ ~#  ~       ~   #~     ~~   #           ~     ~ #  ~     # ~
~~~~~~#~~~ #~# ~~~~~ #~#~## ~~~~#~~~ #~~~~~~ #~#~##~#~ #~~~  #~# ~~~~~~~~~ ##~#~#~~~~~~~ #~#~#~~~#
#~#~#~~~#~  ~~~~# ~~ ~~~~~~ #~#~~~#~~~~#~#~# ~~#~~~~~ ~~~~## ~~~##~#~#~#~# ~~~~~~~# ~#~~~~~#~~~#~~
~~~~~~#~~~ ~~#~~#~#~ #~#~## ~~~~#~~~ #~~~~~~ #~~~#~~# ~#~# ~ #~# ~~~~~~~~~ #~~#~#~~ ~~~#~#~~~#~~~#
#~#~#~~~#~ #~~#~~~~~~~~~~~~ #~#~~~## ~~#~#~# ~~#~#~#~ ~~~~~# ~~~~# ~#~#~~# ~~#~ #~ #~#~~~~~#~~~#~~
~~~~~ #~~~ ~#~~~#~ #~#~#~## ~~~~#~~~ #~~~~~~~#~~~~~~ #~#~#~~ ~#~#  ~~~~~#~ #~~~~~~~ ~~~#~#~~~#~~~#
#~#~#~#~#~#~~~#~~~~~  ~~~~~ #~#~~#~#~~~#~#~#~~~##~#~~ ~~~~~# ~~~~~ ~#~#~~  ~~#~#~#~ ~#~#~~~#~~~#~~
        ~~~~#~ #~ #  ~   #   ~~~    #       ~#~~  ~ #~ #  ~ ~ #~#   ~   ~       ~  ~     # ~ #  ~#
~~~#~#~#~# ~~~~~~~~~#~##~~~ #~#~#~#~~~#~#~#~~~~#~~~~~ ~~ #~~  #~~~ #~#~#~# ~~~~~#~#~~~#~~~~~~~~~~~
#~~~~#~~~~ #~~#~#~#~ ~~~~## ~~~~~~~~#~#~#~~  #~~~##~# ~##~~#~~~~#~~~~#~~~~~##~#~~~~ ~#~ ~#~#~#~#~#
~ #~~~~ ~#~~#~~~~~~~ #~#~~~ #~#~#~#~~~~ ~~~# ~#~ ~~~~~~~~~#~ #~#~~~#~~~#~# ~~~~~#~# ~~~#~ ~~~~~~~~
#~~~#~ #~# ~~~#~#~#~~~~~ ##~~~~~~~~~ #~#~#~~ ~~~###~# ~#~~~~~~~~~# ~~# ~ ~ ~#~#~~~~ ~#~~~~~#~#~#~#
~~# ~~~~~~ ~#~~~~~~~~#~#~~~ #~#~#~~# ~~~~~~~#~#~~~~~~ ~~~#~#~#~#~~~#~~~~#~#~~~~~#~##~~~#~#~~~~~~~~
#~~~#~#~#~#~~~#~#~~#~ ~~~#~ #~~~~#~ ~~#~# #~ ~#~##~#~#~#~~~~~~~~~# ~~~#~~~  #~#~#~~ ~#~#~~~#~#~#~#
  #~  ~    #       ~    ~    #~   ~ ~  ~         ~~  ~  ~# ~    #~~ # ~~#   ~     #~     #~ ~~    
~~~~~#~#~# ~~#~#~ ## ~##~~~#~  ~#~#~ ~~#~#~#~#~~~~#~# ~~~~~~ # ~~~##~~#~~~ #~#~#~~~ #~#~~~~~#~ ~~~
# ~#~~~~~~~~~~ ~~~~~~~~~~#~ #~#~~~~~##~~~~~# ~#~## ~~ ~#~#~# ~~#~# ~~~~~~# ~~~~~ #~#~~~~#~# ~~~#~#
~~~~~~#~#~ ~#~#~#~#~ #~# ~# ~~#~#~#~ ~ #~#~~~~~~~~~~##~~~~~~ # ~~~ #~#~#~~ ~#~#~#~~ ~#~#~~~~~#~~~ 
#~#~#~~~~~#~~~~~~~ ~~~~~~~~ #~~~~~~~~#~~~~~# ~#~##~# ~~#~#~#~~~#~#~~~~~~~#~~~~~~~~#~~~~~ #~#~~~#~~
~~~~~ #~#~ #~#~#~#~# ~#~#~# ~#~# ~#~  #~#~~# ~~~~~~#~#~~~~~~ #~~~~ #~#~#~~~#~ #~#~~ #~#~~~~~~#~~~#
#~#~#~~~~~~~~~~~~~~~ ~~~~~~ ~~~~~~~~#~#~~~ ~~~#~~#~~~ ~#~#~# ~~#~# ~~~~~~# ~~~~~~~# ~~~~#~#~# ~#~~
~~~~~#~#~#~#~~#~#~#~#~#~# ~~#~#~#~#~ ~ ~#~#~#~~~~~~#~ ~~~~~~ #~~~~ ~#~#~#~~#~#~#~#~ ~# ~~~#~#~#~~#
 ~ ~    ~        ~    ~~   ~   ~   ~  ~             ~~ #~  # ~ ~   ~~       ~~~       ~~# ~ ~ ~  ~
#~#~#~#~#~ ~~#~#~#~#~#~#~#~ ~#~#~ ~~ #~#~#~#~####~# ~ ~~~##  ~#~## ~~#~#~# ~#~#~#~~ #~#~ ~~#~#~#~#
~~~~~~~~~~~#~~~~~~~~ #~~~~~#~~~~~~~# ~~~~~~~~~~~~~~~#~ ~~~~~~~~~~~~# ~~~~~ ~~~~~~~# ~~~~~# ~~~~~~~
#~#~#~#~#~  #~#~#~#~ ~~#~#~  #~#~#~~ #~#~#~#~#~#~~#~~ ##~#~# ~#~#~ #~#~#~# ~#~#~#~~~ #~#~~~#~#~#~#
~~~~~~~~~~ ~ ~~~~~~~ #~~~~~~~~~~~~~ ~~~~~~~~ ~~~ ~~~# ~~~~~~~~~ ~~~~~~~~~~~~~~~~~~# ~~~~~ ~~~~~~~~
#~#~# ~#~#~#~#~#~#~# ~#~#~# ~#~#~#~# ~#~#~#~#~#~#~# ~ #~#~#~ #~#~# ~#~#~#~ #~#~#~#~ # ~#~#~#~#~#~#
```

## Day 13: Shuttle Search

[中国剩余定理](https://zh.wikipedia.org/wiki/%E4%B8%AD%E5%9B%BD%E5%89%A9%E4%BD%99%E5%AE%9A%E7%90%86)

## Day 20: Jurassic Jigsaw

Here are my monsters:

```
                                                                                                
                                                                                                
                         O                                                                      
       O    OO    OO    OOO                                                               O     
        O  O  O  O  O  O                                                O    OO    OO    OOO    
                                                        O                O  O  O  O  O  O       
                                      O    OO    OO    OOO                                      
                                       O  O  O  O  O  O                                         
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                        O                                                                       
      O    OO    OO    OOO                                                                      
       O  O  O  O  O  O                                 O                                       
                                      O    OO    OO    OOO                                      
                                       O  O  O  O  O  O                               O         
                                                                    O    OO    OO    OOO        
                                                                     O  O  O  O  O  O           
                                                                                                
                                                                                                
                                                                                                
                                                                                        O       
                                                                      O    OO    OO    OOO      
                                                                       O  O  O  O  O  O         
                                                                                                
                                                                                                
                                                                                                
                                            O                                                   
                          O    OO    OO    OOO                                                  
                           O  O  O  O  O  O                                                     
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                    O                                                           
                  O    OO    OO    OOO                                                          
                   O  O  O  O  O  O                                                             
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                         O                                                      
                       O    OO    OO    OOO                                                     
                        O  O  O  O  O  O                                                        
                                                               O                                
                                             O    OO    OO    OOO                               
                                              O  O  O  O  O  O                                  
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                                            O   
                                                                      O   O    OO    OO    OOO  
                                                    O    OO    OO    OOO   O  O  O  O  O  O     
                                                     O  O  O  O  O  O                           
                                                                                                
                    O                                                                   O       
  O    OO    OO    OOO                                                O    OO    OO    OOO      
   O  O  O  O  O  O                                                    O  O  O  O  O  O         
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                  O                             
                                                O    OO    OO    OOO                            
                                                 O  O  O  O  O  O                               
                                                                                                
                                                                                                
                                                                                  O             
                                                                O    OO    OO    OOO            
                                                                 O  O  O  O  O  O               
                                                                                                
                                                                                                
                                                                         O                      
                       O                               O    OO    OO    OOO                     
     O    OO    OO    OOO                               O  O  O  O  O  O                        
      O  O  O  O  O  O                                                                          
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                                                                
                                                       O                                        
                                     O    OO    OO    OOO                                       
                                      O  O  O  O  O  O                                          
                                                                                                
                                                                                                
```

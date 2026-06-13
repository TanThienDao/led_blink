To find which pins control the LEDs for the "compass" on the STM32F3 Discovery board, you can look at the \*\*schematics\*\* or the board's \*\*User Manual\*\*.

### 1\. The Pin Mapping
The compass LEDs are all connected to \*\*GPIO Port E\*\*. The specific pins are:  
\* \*\*North:\*\* PE9 (Red)  
\* \*\*North-East:\*\* PE10 (Orange)  
\* \*\*East:\*\* PE11 (Yellow)  
\* \*\*South-East:\*\* PE12 (Green)  
\* \*\*South:\*\* PE13 (Red)  
\* \*\*South-West:\*\* PE14 (Orange)  
\* \*\*West:\*\* PE15 (Yellow)  
\* \*\*North-West:\*\* PE8 (Green)

![stm32f3_reference_manual_6.4LEDs.png](stm32f3_reference_manual_6.4LEDs.png)

\### 2. How to set them in \`moder\`  
Since the LEDs are on pins \*\*8 through 15\*\*, you need to set all of those pins to \`output()\` in the \`moder\` register.

In your code, you currently only have pin 8 set:  
\`\`\`rust  
gpioe.moder.modify(|\_, w| {  
    w.moder8().output() // Only pin 8 is configured as output  
});  
\`\`\`

To turn on the whole compass, you should update it to include all pins from 8 to 15:  
\`\`\`rust  
gpioe.moder.modify(|\_, w| {  
    w.moder8().output();  
    w.moder9().output();  
    w.moder10().output();  
    w.moder11().output();  
    w.moder12().output();  
    w.moder13().output();  
    w.moder14().output();  
    w.moder15().output()  
});  
\`\`\`

\### 3. Summary of LED Bits

| LED Color | Compass Direction | Pin | Register bit |
| :--- | :--- | :--- | :--- |
| Green | NW  | PE8 | moder8 / odr8 |
| Red | N   | PE9 | moder9 / odr9 |
| Orange | NE  | PE10 | moder10 / odr10 |
| Yellow | E   | PE11 | moder11 / odr11 |
| Green | SE  | PE12 | moder12 / odr12 |
| Red | S   | PE13 | moder13 / odr13 |
| Orange | SW  | PE14 | moder14 / odr14 |
| Yellow | W   | PE15 | moder15 / odr15 |

If you only set \`moder8().output()\`, only the \*\*North-West (Green)\*\* LED will actually light up when you write to the \`odr\` register, even if you set all bits in \`odr\`. The other pins will remain in "Input" mode and won't drive any current to the LEDs.
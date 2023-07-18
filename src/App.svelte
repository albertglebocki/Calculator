<script lang="ts">
  /* Setup */
  import { appWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/tauri';

  async function newWindow() {
    await invoke('create_new_window');
  }

  document.addEventListener('contextmenu', event => event.preventDefault());

  /* Calculator Logic */
  let expressionString: string = "";

  function updateExpression(value: string) {
    expressionString += value;
  }

  function evaluateExpression(expression: string) {
    expressionString = Function(`'use strict'; return (${expression})`)();
  }
</script>

<div data-tauri-drag-region class="titlebar">
  <div data-tauri-drag-region class="navigation">
    <button id="close" on:click={appWindow.close}></button>
    <button id="minimize" on:click={appWindow.minimize}></button>
    <button id="maximize" on:click={newWindow}></button>
  </div>

  <span data-tauri-drag-region class="title">Calculator</span>
</div>

<div class="calculator">
  <input type="text" id="result" bind:value={expressionString} readonly>

  <div class="row">
    <input type="button" class="button dark" value="AC" on:click={() => {expressionString = ""}}>
    <input type="button" class="button dark" value="+/-" on:click={() => {if (expressionString != "") { expressionString = `-(${expressionString})`}}}>
    <input type="button" class="button dark" value="&#37;" on:click={() => {updateExpression("*0.01")}}>
    <input type="button" class="button orange" value="/" on:click={() => {updateExpression("/")}}>
  </div>

  <div class="row">
    <input type="button" class="button" value="7" on:click={() => {updateExpression("7")}}>
    <input type="button" class="button" value="8" on:click={() => {updateExpression("8")}}>
    <input type="button" class="button" value="9" on:click={() => {updateExpression("9")}}>
    <input type="button" class="button orange" value="x" on:click={() => {updateExpression("*")}}>
  </div>
  
  <div class="row">
    <input type="button" class="button" value="4" on:click={() => {updateExpression("4")}}>
    <input type="button" class="button" value="5" on:click={() => {updateExpression("5")}}>
    <input type="button" class="button" value="6" on:click={() => {updateExpression("6")}}>
    <input type="button" class="button orange" value="-" on:click={() => {updateExpression("-")}}>
  </div>

  <div class="row">
    <input type="button" class="button" value="1" on:click={() => {updateExpression("1")}}>
    <input type="button" class="button" value="2" on:click={() => {updateExpression("2")}}>
    <input type="button" class="button" value="3" on:click={() => {updateExpression("3")}}>
    <input type="button" class="button orange" value="+" on:click={() => {updateExpression("+")}}>
  </div>

  <div class="row">
    <input type="button" class="button big" value="0" on:click={() => {updateExpression("0")}}>
    <input type="button" class="button dark" value="." on:click={() => {updateExpression(".")}}>
    <input type="button" class="button orange" value="=" on:click={() => {evaluateExpression(expressionString)}}>
  </div>
</div>

<style>
  .titlebar {
    width: calc(100% - 1rem);
    padding: .5rem;
    cursor: default;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    border-top-left-radius: 10px;
    border-top-right-radius: 10px;
    background-color: #FCFCFC;
    border-bottom: 1px solid #F2F2F2;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
  }
  .navigation {
    left: .75rem;
    position: absolute;
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: .3rem;
  }
  .navigation > button {
    aspect-ratio: 1 / 1;
    height: 12px;
    border-radius: 50%;
    border: none;
    cursor: pointer;
    transition: transform 250ms, filter 350ms;
  }
  .navigation > button:hover {
    transform: scale(1.035);
    filter: contrast(125%) brightness(95%);
  }
  .navigation > button:active {
    transform: scale(1.25);
  }
  #close {
    background-color: #ff6057;
  }
  #minimize {
    background-color: #ffbd34;
  }
  #maximize {
    background-color: #30c13c;
  }
  .title {
    font-size: 1rem;
    font-weight: 500;
    -webkit-user-select: none;  
    -moz-user-select: none;    
    -ms-user-select: none;      
    user-select: none;
  }
  .calculator {
    display: grid;
    place-items: center;
    gap: .65rem;
  }
  #result {
    width: calc(100% - 4rem);
    padding: .25rem 2rem;
    height: 4rem;
    border: none;
    cursor: default;
    font-size: 3.35rem;
    text-align: right;
    background-color: #FFFFFF;
  }
  #result:focus {
    outline: none;
  }
  .row {
    width: calc(100% - 1.3rem - 2.25rem);
    max-width: calc(100% - 1.3rem - 2.25rem);
    display: flex;
    flex-direction: row;
    gap: .65rem;
    justify-content: center;
  }
  .button {
    aspect-ratio: 1 / 1;
    height: 62px;
    width: 62px;
    font-size: 1.45rem;
    font-weight: 500;
    border-radius: 100%;
    border: none;
    display: grid;
    place-items: center;
    background-color: #F1F1F1;
    cursor: pointer;
    transition: transform 350ms, filter 350ms;
  }
  .button:hover {
    filter: brightness(.925);
  }
  .button:active {
    transform: scale(1.1);
    transform-origin: center;
  }
  .button.big {
    flex-grow: 1;
    height: 62px;
    border-radius: 5rem;
  }
  .button.dark {
    background-color: #E4E4E4;
  }
  .button.orange {
    background-color: #fca94c;
  }
</style>
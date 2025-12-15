
It is a common problem, sometime we forget to record an expense. And a week after when we see the statement, we see we are down by some amount. Then we have to tally the bank app statements to verify what we missed. This doesn't have to always be the case. With Finrup, you can take it to the next level with the following suggestions so that the tracking of expenses is more easier. Lets dive into few of the advanced features of Finrup.

## Widgets

Most of the time, human nature is to forget things that takes a little effort. We need some visual reminder to remember we missed something. Widgets plays a vital role as gentle reminders to users. With Finrup, you can add a widget in your homescreen which reminds you to add expenses, income and open the specific feature easily. To add widgets, simply do the following steps:

- **Enter jiggle mode** : On your homescreen, press and hold any item or empty space until the apps starts to jiggle. 
- **Tap Edit**: Tap the '+' (Add)  button in the top-left corner of the screen if availale or tap edit button and tap on Add widgets option there. 
- **Find Finrup**: Either enter Finrup in search bar or scroll down to find the finrup app name among the widgets. 
- **Choose Widget**: Swipe left to see different widget sizes and functions and select the one which feels suitable for you. 
- **Add widget**: Click on Add widget once you are on right screen and drag it to where you feel it feels appropriate. 
![Finrup Widget Selection](/img/posts/tech/finrup/Finrup-App-Widgets.jpg)

From the widgets, when you click on the add expense, you will directly open the form to add expense which reduces the number of steps to record a transaction.


## Siri

Siri can be a powerful tool when used properly, despite its occasional issues. You can check the commands available for Finrup directly through the app by going to the voice command help in the app's settings.

The real magic, however, lies one step further with App Shortcuts and Siri. Once you identify your most frequent payments, you can use the App Shortcuts provided by Finrup to create custom voice commands.

### Case: Dinner or other frequent expenses

Let's say you frequently have dinner expenses, but the amount usually varies, so you can't use the recurring transactions feature. In such a case, you can create an App Shortcut for this scenario.

To create a shortcut:
- Open the Shortcuts app
- Tap the plus icon in the top right
- Tap "New Shortcut" and rename it to something obvious like "Record dinner expense"
- Search for "Finrup" in the search for actions
- Select "Add expense" from the actions
- If you know the account will always be the same:
    - Tap on the greyed out "Account" field and select the account. Otherwise, leave it empty
- Tap the right-pointing arrow and select the category
- Add some notes (optional)

That's all! Next time, you can just say "Record dinner expense" and Siri will prompt you for any missing information, making it easy to record the expense.

You can create similar shortcuts for other frequent expenses like cab fare, rent, and so on. You can even do the same for transfers or income entries.

## Billings

One of the most useful scenarios for automation is handling receipts and bills. Let's say you're shopping at a supermarket, bought many items, and received a receipt. You have two options:

- Enter all expenses as a single sum entry (e.g., "I spent this much on groceries")
- Enter each item separately, one by one

Both approaches have drawbacks: the first doesn't help when you want to look up a specific item's price later, while the second is time-consuming. This is where automation comes to the rescue.


If your phone supports Apple Intelligence, you can parse the receipt and add the items automatically.

Here's how to create an example shortcut:
- Open the Shortcuts app
- Tap the plus icon in the top right
- Tap "New Shortcut" and rename it to something obvious like "Parse the bill"
- Add a "Set variable" action in the following way:
    - Set variable `photoInput` to `Shortcut Input`
- Change the shortcut input to receive `Images and media` from `Share Sheet, What's On Screen` and set the app to `Continue` if there is no input
- Add an "If" condition: if `photoInput` `does not have any value`
    - Inside it, add an action to Take `1 photo` with the `Back` camera
    - Set variable `photoInput` to the `Photo` taken in the previous step
- Use the `Cloud` model to access Apple Intelligence with the following prompt:
  > Go through the receipt and identify the list of items. Return a list of items with their prices, with name and amount separated by a comma. The amount will usually be at the end of the row. Ignore the quantity field from the receipt
  > `photoInput`

  Set the output to list
- Set the variable `listOfItems` to the `Response` from the cloud call above
- Select `Account` (Finrup). Set the account field to empty to prompt for input
- Select `Expense Category` (Finrup). Set this to empty as well
- Add a "Repeat with each item" loop for the list of items:
    - For each loop, Split `Repeat Item` by `Custom` `,`
    - Get `First Item` from `Split Text`
    - Set variable `description` to `Item from List`
    - Get `Item at Index` `2` from `Split Text`
    - Set variable `amount` to `Item from List`
    - Add `amount` expense to `Select account` (Set category to `Select expense category` and note to `description`)
- Delete `photoInput` if desired

I know this all looks daunting. The simpler solution is to download the shortcut directly:

**[Download the Bill Parsing Shortcut](https://www.icloud.com/shortcuts/98dde59d83c14bb2bff50da230a1d706)** - Tap to add it to your Shortcuts library instantly.

## Automatic messages

Another helpful automation I found is to make use of automatic messages. If your bank account sends you a text message or email for the notification of any transaction you can use it to prepare a good automation. 

Lets say one of my bank account sends me following text message whenever I make an expense: 

> Your 166###29001 has been Debited by NPR 610.00 on 13/12/2025 15:03:16, Remarks: vegetables..
> ABC BANK

This is how you can make use of this:

- Open **Shortcut** App
- Tap on the **Automation** tab from the bottom bar.
- Now, click on plus icon on top right to add an Automation
- Select `Email` or `Message` depending on how you get your notification.
    - If you select `Email`, select appropriate value in *Sender*, *Subject Contains*, *Recipient* and so on to filter out the notification email. Use a value that usually exists in Subject when the expense is notified. 
    - If you select `Message` , select *Sender* if applicable and *message contains* to appropriate value. 
- In above sample, I am selecting, `Message` and `Your 166###29001 has been Debited by` to *message contains* to identify the notification.
- Then select `Create New Shortcut` with it. 
- Select `Finrup` through the app filter and select `Process transaction message ` for such.
- Change the `message field` to `Shortcut Input`. From the more settings, select the default account to your account, default category if applicable. And toggle skip confirmation.
- Make sure to change the automation to run immediately.

This way, whenever a new transaction notification is received, the transaction is recorded automatically to the app. You can review it later if the Apple Intelligence got it right or not.

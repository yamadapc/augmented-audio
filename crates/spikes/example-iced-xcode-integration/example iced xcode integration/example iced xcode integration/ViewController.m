//
//  ViewController.m
//  example iced xcode integration
//
//  Created by Pedro Tacla Yamada on 25/7/21.
//

#import "ViewController.h"
#import "rust_bridge.h"

@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];

    NSView* view = [self view];
    attach_to_view(view);
}


- (void)setRepresentedObject:(id)representedObject {
    [super setRepresentedObject:representedObject];

    // Update the view, if already loaded.
}


@end
